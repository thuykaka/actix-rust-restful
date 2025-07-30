use actix_web::{App, HttpResponse, HttpServer, middleware, middleware::Logger, web};
use dotenv::dotenv;
use env_logger::Env;

use actix_rust_restful::{config, controllers, repositories, services};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database_mockup = repositories::database_mockup::Database::new();
    let todo_service = services::todo_service::TodoService::new(database_mockup);

    let todo_data_state = web::Data::new(todo_service);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "1.0")))
            .app_data(todo_data_state.clone())
            .configure(controllers::home_controller::config)
            .configure(controllers::todo_controller::config)
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().json(serde_json::json!({
                    "code": "NOT_FOUND",
                    "message": "Not found"
                }))
            }))
    })
    .bind(("0.0.0.0", *config::constants::PORT))?
    .run()
    .await
}
