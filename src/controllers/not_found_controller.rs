use actix_web::{HttpResponse, Responder};

pub async fn not_found_handler() -> impl Responder {
    HttpResponse::NotFound().json(serde_json::json!({
        "code": "NOT_FOUND",
        "message": "Not found"
    }))
}
