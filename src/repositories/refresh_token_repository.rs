use crate::models::errors::Error;
use chrono::Utc;
use entity::t_refresh_token;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

#[derive(Clone)]
pub struct RefreshTokenRepository {
    pub db: DatabaseConnection,
}

impl RefreshTokenRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_refresh_token(
        &self,
        refresh_token: t_refresh_token::ActiveModel,
    ) -> Result<t_refresh_token::Model, Error> {
        let refresh_token = refresh_token.insert(&self.db).await?;
        Ok(refresh_token)
    }

    pub async fn get_refresh_token(
        &self,
        token: Uuid,
    ) -> Result<Option<t_refresh_token::Model>, Error> {
        let refresh_token = t_refresh_token::Entity::find()
            .filter(t_refresh_token::Column::Token.eq(token))
            .filter(t_refresh_token::Column::ExpiredAt.gt(Utc::now()))
            .one(&self.db)
            .await?;
        Ok(refresh_token)
    }
}
