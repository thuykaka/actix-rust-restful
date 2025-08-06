use crate::{
    config,
    daos::redis_dao::{RedisDao, RedisOperations},
    models::{
        errors::Error,
        request::{self, GetAllTodosRequest, UpdateTodoRequest},
        response::{CommonResponse, GetAllTodosResponse},
    },
    repositories::todo_repository::TodoRepository,
    services::http_request_service::{HttpRequestError, HttpRequestService},
};
use chrono::Utc;
use entity::t_todos;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;

#[derive(Clone)]
pub struct TodoService {
    pub todo_repository: TodoRepository,
    pub redis_dao: RedisDao,
    pub http_request_service: HttpRequestService,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ExternalTodo {
    id: u32,
    todo: String,
    completed: bool,
    #[serde(rename = "userId")]
    user_id: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExternalTodosResponse {
    todos: Vec<ExternalTodo>,
    total: u32,
    skip: u32,
    limit: u32,
}

impl TodoService {
    pub fn new(
        todo_repository: TodoRepository,
        redis_dao: RedisDao,
    ) -> Result<Self, HttpRequestError> {
        Ok(Self {
            todo_repository,
            redis_dao,
            http_request_service: HttpRequestService::new()?,
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn create_todo(
        &self,
        body: request::CreateTodoRequest,
        user_id: Uuid,
    ) -> Result<t_todos::Model, Error> {
        let start_time = Instant::now();

        let todo_model = body.into_active_model(user_id);
        let todo = self.todo_repository.create(todo_model).await?;

        log::info!("create_todo took {}ms", start_time.elapsed().as_millis());
        Ok(todo)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_all_todos(
        &self,
        params: GetAllTodosRequest,
    ) -> Result<GetAllTodosResponse, Error> {
        log::info!("Getting all todos with params: {:?}", &params);

        let limit = params.page_size.unwrap_or(*config::DEFAULT_PAGE_SIZE);
        let offset = (params.page.unwrap_or(*config::DEFAULT_PAGE) - 1) * limit;

        let todos = self
            .todo_repository
            .get_all_todos(limit, offset, params.search)
            .await?;

        let total = todos.len();
        let total_pages = (total as f64 / limit as f64).ceil() as usize;

        Ok(GetAllTodosResponse {
            total,
            total_pages,
            data: todos,
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_todo_by_id(&self, id: String) -> Result<t_todos::Model, Error> {
        let id =
            Uuid::parse_str(&id).map_err(|_| Error::BadRequest("Invalid todo id".to_string()))?;

        let todo = self
            .todo_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::BadRequest(format!("Todo with id {} not found", id)))?;

        Ok(todo)
    }

    #[tracing::instrument(skip(self))]
    pub async fn delete_todo(&self, id: String) -> Result<CommonResponse<String>, Error> {
        let id =
            Uuid::parse_str(&id).map_err(|_| Error::BadRequest("Invalid todo id".to_string()))?;

        self.todo_repository.delete(id).await?;

        Ok(CommonResponse {
            message: format!("Deleted todo id {} successfully", id),
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn update_todo(
        &self,
        id: String,
        body: UpdateTodoRequest,
    ) -> Result<t_todos::Model, Error> {
        let id =
            Uuid::parse_str(&id).map_err(|_| Error::BadRequest("Invalid todo id".to_string()))?;

        let mut todo_active_model = self
            .todo_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::BadRequest(format!("Todo with id {} not found", id)))?
            .into_active_model();

        if let Some(title) = body.title {
            todo_active_model.title = Set(title);
        }

        if let Some(description) = body.description {
            todo_active_model.description = Set(description);
        }

        if let Some(completed) = body.completed {
            todo_active_model.completed = Set(completed);
        }

        todo_active_model.updated_at = Set(Utc::now().into());
        let updated_todo = self.todo_repository.update(todo_active_model).await?;

        Ok(updated_todo)
    }

    // Test call external API with cache
    pub async fn get_external_data(&self) -> Result<ExternalTodosResponse, Error> {
        let key = "EXTERNAL_TODOS";

        let cached_value = match self.redis_dao.get::<ExternalTodosResponse>(key).await {
            Ok(value) => value,
            Err(_) => None,
        };

        if let Some(cached_data) = cached_value {
            log::info!("cache hit for external todos");
            return Ok(cached_data);
        }

        let resp = self
            .http_request_service
            .get::<ExternalTodosResponse>("https://dummyjson.com/todos")
            .await?;

        let redis_dao = self.redis_dao.clone();
        let resp_clone = resp.clone();
        tokio::spawn(async move {
            match redis_dao.set(key, &resp_clone).await {
                Ok(_) => log::info!("successfully cached external todos"),
                Err(e) => log::error!("failed to set value: {:?}", e),
            }
        });

        Ok(resp)
    }
}
