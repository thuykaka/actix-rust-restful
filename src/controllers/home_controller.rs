use actix_web::{HttpResponse, Responder, get, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "ping": "pong"
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}
