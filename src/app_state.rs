use crate::{config, models::errors::Error, services::auth_service::AuthService};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub struct AppState {
    pub auth_service: AuthService,
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

        let auth_service = AuthService::new(db_connection);

        Ok(AppState { auth_service })
    }
}
