use async_trait::async_trait;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Undefined = b'a' as isize,
    Null = b'b' as isize,
    Boolean = b'0' as isize,
    String = b'1' as isize,
    Number = b'2' as isize,
    Date = b'3' as isize,
    Object = b'4' as isize,
}

impl DataType {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a' => Some(DataType::Undefined),
            'b' => Some(DataType::Null),
            '0' => Some(DataType::Boolean),
            '1' => Some(DataType::String),
            '2' => Some(DataType::Number),
            '3' => Some(DataType::Date),
            '4' => Some(DataType::Object),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Date(chrono::DateTime<chrono::Utc>),
    Object(serde_json::Value),
}

impl Value {
    pub fn to_string_with_type(&self) -> String {
        match self {
            Value::Undefined => format!("a"),
            Value::Null => format!("b"),
            Value::Boolean(b) => format!("0{}", if *b { "1" } else { "0" }),
            Value::String(s) => format!("1{}", s),
            Value::Number(n) => format!("2{}", n),
            Value::Date(d) => format!("3{}", d.timestamp_millis()),
            Value::Object(obj) => format!("4{}", serde_json::to_string(obj).unwrap()),
        }
    }

    pub fn from_string_with_type(
        s: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if s.is_empty() {
            return Ok(Value::Null);
        }

        let type_char = s.chars().next().unwrap();
        let content = &s[1..];

        match DataType::from_char(type_char) {
            Some(DataType::Undefined) => Ok(Value::Undefined),
            Some(DataType::Null) => Ok(Value::Null),
            Some(DataType::Boolean) => {
                let bool_val = content == "1";
                Ok(Value::Boolean(bool_val))
            }
            Some(DataType::String) => Ok(Value::String(content.to_string())),
            Some(DataType::Number) => {
                let num = content.parse::<f64>()?;
                Ok(Value::Number(num))
            }
            Some(DataType::Date) => {
                let timestamp = content.parse::<i64>()?;
                let date = chrono::DateTime::from_timestamp_millis(timestamp)
                    .ok_or("Invalid timestamp")?;
                Ok(Value::Date(date))
            }
            Some(DataType::Object) => {
                let obj = serde_json::from_str(content)?;
                Ok(Value::Object(obj))
            }
            None => Err("Invalid data type".into()),
        }
    }
}

#[async_trait]
pub trait RedisOperations: Send + Sync {
    // Nếu dùng async thì phải có async trait và trả về Box<dyn Error + Send + Sync>
    // dyn Error là error trait, Send + Sync là để cho phép gửi và nhận error trên thread khác
    async fn set<T>(&self, key: &str, value: T) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        T: Serialize + Send + Sync;

    async fn get<T>(&self, key: &str) -> Result<Option<T>, Box<dyn Error + Send + Sync>>
    where
        T: for<'de> Deserialize<'de> + Send + Sync;
}

#[derive(Clone)]
pub struct RedisDao {
    pub connection: ConnectionManager,
}

impl RedisDao {
    pub fn new(connection: ConnectionManager) -> Self {
        Self { connection }
    }

    pub async fn set_value(
        &mut self,
        key: &str,
        value: &Value,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let value_string = value.to_string_with_type();
        let _: () = self.connection.set(key, value_string).await?;
        Ok(())
    }

    pub async fn get_value(
        &mut self,
        key: &str,
    ) -> Result<Option<Value>, Box<dyn Error + Send + Sync>> {
        let result: Option<String> = self.connection.get(key).await?;

        match result {
            Some(value_string) => {
                let value = Value::from_string_with_type(&value_string)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    pub async fn set_typed<T>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        T: Serialize + Send + Sync,
    {
        let value_enum = self.serialize_to_value(value).await?;
        self.set_value(key, &value_enum).await
    }

    pub async fn get_typed<T>(
        &mut self,
        key: &str,
    ) -> Result<Option<T>, Box<dyn Error + Send + Sync>>
    where
        T: for<'de> Deserialize<'de> + Send + Sync,
    {
        let value_enum = self.get_value(key).await?;
        match value_enum {
            Some(value) => {
                let typed_value = self.deserialize_from_value::<T>(value).await?;
                Ok(Some(typed_value))
            }
            None => Ok(None),
        }
    }

    async fn serialize_to_value<T>(&self, value: T) -> Result<Value, Box<dyn Error + Send + Sync>>
    where
        T: Serialize + Send + Sync,
    {
        let json_value = serde_json::to_value(value)?;

        match json_value {
            serde_json::Value::Null => Ok(Value::Null),
            serde_json::Value::Bool(b) => Ok(Value::Boolean(b)),
            serde_json::Value::String(s) => Ok(Value::String(s)),
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Ok(Value::Number(f))
                } else {
                    Err("Invalid number".into())
                }
            }
            serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                Ok(Value::Object(json_value))
            }
        }
    }

    async fn deserialize_from_value<T>(
        &self,
        value: Value,
    ) -> Result<T, Box<dyn Error + Send + Sync>>
    where
        T: for<'de> Deserialize<'de> + Send + Sync,
    {
        let json_value = match value {
            Value::Null => serde_json::Value::Null,
            Value::Boolean(b) => serde_json::Value::Bool(b),
            Value::String(s) => serde_json::Value::String(s),
            Value::Number(n) => serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap()),
            Value::Date(d) => serde_json::Value::String(d.to_rfc3339()),
            Value::Object(obj) => obj,
            Value::Undefined => serde_json::Value::Null,
        };

        let typed_value = T::deserialize(json_value)?;
        Ok(typed_value)
    }

    pub async fn del(&mut self, key: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _: usize = self.connection.del(key).await?;
        Ok(())
    }

    pub async fn exists(&mut self, key: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let result: i32 = self.connection.exists(key).await?;
        Ok(result > 0)
    }

    pub async fn expire(
        &mut self,
        key: &str,
        seconds: u64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _: bool = self.connection.expire(key, seconds as i64).await?;
        Ok(())
    }

    pub async fn ttl(&mut self, key: &str) -> Result<i32, Box<dyn Error + Send + Sync>> {
        let result: i32 = self.connection.ttl(key).await?;
        Ok(result)
    }
}

#[async_trait]
impl RedisOperations for RedisDao {
    async fn set<T>(&self, key: &str, value: T) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        T: Serialize + Send + Sync,
    {
        let value_enum = self.serialize_to_value(value).await?;
        let value_string = value_enum.to_string_with_type();
        let mut conn = self.connection.clone();
        let _: () = conn.set(key, value_string).await?;
        Ok(())
    }

    async fn get<T>(&self, key: &str) -> Result<Option<T>, Box<dyn Error + Send + Sync>>
    where
        T: for<'de> Deserialize<'de> + Send + Sync,
    {
        let mut conn = self.connection.clone();
        let result: Option<String> = conn.get(key).await?;

        match result {
            Some(value_string) => {
                let value = Value::from_string_with_type(&value_string)?;
                let typed_value = self.deserialize_from_value::<T>(value).await?;
                Ok(Some(typed_value))
            }
            None => Ok(None),
        }
    }
}
