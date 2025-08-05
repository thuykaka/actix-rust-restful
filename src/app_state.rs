use crate::{
    config,
    models::errors::Error,
    repositories::{
        refresh_token_repository::RefreshTokenRepository, todo_repository::TodoRepository,
        user_repository::UserRepository,
    },
    services::{auth_service::AuthService, todo_service::TodoService},
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub struct AppState {
    pub auth_service: AuthService,
    pub todo_service: TodoService,
}

impl AppState {
    pub async fn init_db_and_services() -> Result<Self, Error> {
        log::info!("Connecting to PorstgreSQL...");

        let mut options = ConnectOptions::new(config::DATABASE_URL.to_owned());

        options
            .max_connections(*config::DB_MAX_CONNECTIONS)
            .connect_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(30))
            .max_lifetime(Duration::from_secs(30))
            .sqlx_logging(false)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db_connection: DatabaseConnection = Database::connect(options).await?;

        log::info!("Connected to PorstgreSQL");

        let user_repository = UserRepository::new(db_connection.clone());
        let refresh_token_repository = RefreshTokenRepository::new(db_connection.clone());
        let todo_repository = TodoRepository::new(db_connection.clone());

        let auth_service = AuthService::new(user_repository, refresh_token_repository);
        let todo_service = TodoService::new(todo_repository);

        Ok(AppState {
            auth_service,
            todo_service,
        })
    }
}
