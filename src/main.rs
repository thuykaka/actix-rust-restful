mod app_state;
mod config;
mod controllers;
mod middlewares;
mod models;
mod repositories;
mod services;
mod utils;
mod validators;

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    middleware::{self as actix_web_middleware, Logger},
    web,
};
use actix_web_validation::validator::ValidatorErrorHandlerExt;
use app_state::AppState;
use controllers::{auth_controller, home_controller, not_found_controller, todo_controller};
use dotenv::dotenv;
use env_logger::Env;
use middlewares::rate_limit_middleware::rate_limiter_middleware;
use models::errors::Error;
use std::sync::Arc;
use utils::{request_handler::json_error_handler, response_handler::validator_error_handler};

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
            .wrap(actix_web_middleware::Compress::default())
            .wrap(Cors::permissive()) // allow all origins
            .wrap(actix_web_middleware::DefaultHeaders::new().add(("x-powered-by", "actix-web")))
            .wrap(rate_limiter_middleware())
            .validator_error_handler(Arc::new(validator_error_handler))
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
            .app_data(app_data.clone())
            .configure(home_controller::config)
            .configure(auth_controller::config)
            .configure(todo_controller::config)
            .default_service(web::route().to(not_found_controller::not_found_handler))
    })
    .workers(2)
    .bind(("0.0.0.0", *config::PORT))?
    .run()
    .await
    .map_err(|error| {
        log::error!("Server start failed: {}", error);
        Error::ServerStartFailed(error)
    })
}
