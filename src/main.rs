mod app_state;
mod config;
mod controllers;
mod middlewares;
mod models;
mod services;
mod utils;

use actix_web::{App, HttpServer, middleware, middleware::Logger, web};
use app_state::AppState;
use controllers::{auth_controller, home_controller, not_found_controller};
use dotenv::dotenv;
use env_logger::Env;
use models::errors::Error;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Init database connection and services
    let app_state = AppState::init_db_and_services().await?;

    // App data
    let app_data = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new().add(("x-powered-by", "actix-web")))
            .app_data(app_data.clone())
            .configure(home_controller::config)
            .configure(auth_controller::config)
            .default_service(web::route().to(not_found_controller::not_found_handler))
    })
    .bind(("0.0.0.0", *config::PORT))?
    .run()
    .await
    .map_err(|error| {
        log::error!("Server start failed: {}", error);
        Error::ServerStartFailed(error)
    })
}
