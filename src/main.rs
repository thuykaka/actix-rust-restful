use actix_rust_restful::{
    config,
    controllers::{auth_controller, home_controller, todo_controller},
    models::app_state::AppState,
};
use actix_web::{App, HttpResponse, HttpServer, middleware, middleware::Logger, web};
use dotenv::dotenv;
use env_logger::Env;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let mut opt = ConnectOptions::new(config::constants::DATABASE_URL.to_owned());
    opt.max_connections(100).sqlx_logging(false);

    let db_connection: DatabaseConnection = Database::connect(opt).await.unwrap();

    let app_state = web::Data::new(AppState { db: db_connection });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "1.0")))
            .app_data(app_state.clone())
            .configure(home_controller::config)
            .configure(todo_controller::config)
            .configure(auth_controller::config)
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().json(serde_json::json!({
                    "code": "NOT_FOUND",
                    "message": "Not found"
                }))
            }))
    })
    // .workers(2)
    .bind(("0.0.0.0", *config::constants::PORT))?
    .run()
    .await
}
