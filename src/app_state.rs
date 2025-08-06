use std::time::Duration;

use crate::{
    config,
    daos::redis_dao::RedisDao,
    models::errors::Error,
    repositories::{
        refresh_token_repository::RefreshTokenRepository, todo_repository::TodoRepository,
        user_repository::UserRepository,
    },
    services::{auth_service::AuthService, todo_service::TodoService},
};
use redis::{Client, aio::ConnectionManager};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct AppState {
    pub auth_service: AuthService,
    pub todo_service: TodoService,
}

impl AppState {
    pub async fn new() -> Result<Self, Error> {
        log::info!("Initializing application state");

        // Create database connection
        let db_connection = Self::create_database_connection().await?;

        // Create Redis connection
        let redis_connection = Self::create_redis_connection().await?;
        let redis_dao = RedisDao::new(redis_connection.clone());

        // Create repositories
        let (user_repo, refresh_repo, todo_repo) = Self::create_repositories(&db_connection);

        // Create services
        let (auth_service, todo_service) =
            Self::create_services(user_repo, refresh_repo, todo_repo, redis_dao)?;

        log::info!("Application state initialized successfully");

        Ok(AppState {
            auth_service,
            todo_service,
        })
    }

    async fn create_redis_connection() -> Result<ConnectionManager, Error> {
        log::info!("Connecting to Redis...");
        let client = Client::open(config::REDIS_URL.to_string())
            .map_err(|e| Error::InternalServerError(e.to_string()))?;

        let manager = ConnectionManager::new(client)
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;

        log::info!("Connected to Redis successfully");
        Ok(manager)
    }

    async fn create_database_connection() -> Result<DatabaseConnection, Error> {
        log::info!("Connecting to PostgreSQL...");

        let mut options = ConnectOptions::new(config::DATABASE_URL.to_string());
        options
            .max_connections(*config::DB_MAX_CONNECTIONS)
            .connect_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(30))
            .max_lifetime(Duration::from_secs(30))
            .sqlx_logging(false)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db_connection = Database::connect(options).await?;

        log::info!("Connected to PostgreSQL successfully");
        Ok(db_connection)
    }

    fn create_repositories(
        db_connection: &DatabaseConnection,
    ) -> (UserRepository, RefreshTokenRepository, TodoRepository) {
        let user_repository = UserRepository::new(db_connection.clone());
        let refresh_token_repository = RefreshTokenRepository::new(db_connection.clone());
        let todo_repository = TodoRepository::new(db_connection.clone());

        (user_repository, refresh_token_repository, todo_repository)
    }

    fn create_services(
        user_repo: UserRepository,
        refresh_repo: RefreshTokenRepository,
        todo_repo: TodoRepository,
        redis_dao: RedisDao,
    ) -> Result<(AuthService, TodoService), Error> {
        let auth_service = AuthService::new(user_repo, refresh_repo);
        let todo_service = TodoService::new(todo_repo, redis_dao)?;

        Ok((auth_service, todo_service))
    }
}
