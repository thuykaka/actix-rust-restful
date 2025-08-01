use crate::models::errors::Error;
use actix_web::HttpResponse;
use serde_json::json;

#[allow(unused_imports)]
pub trait ErrorToHttp {
    fn to_http_response(&self) -> HttpResponse;
}

impl ErrorToHttp for Error {
    fn to_http_response(&self) -> HttpResponse {
        match self {
            Error::EmailAlreadyExists => HttpResponse::Conflict().json(json!({
                "message": "Email already exists"
            })),
            Error::InvalidEmail => HttpResponse::BadRequest().json(json!({
                "message": "Invalid email format"
            })),
            Error::PasswordTooShort => HttpResponse::BadRequest().json(json!({
                "message": "Password too short"
            })),
            Error::InvalidId(id) => HttpResponse::BadRequest().json(json!({
                "message": format!("Invalid ID: {}", id)
            })),
            Error::NotFound(message) => HttpResponse::NotFound().json(json!({
                "message": message
            })),
            Error::InvalidCredentials => HttpResponse::Unauthorized().json(json!({
                "message": "Invalid credentials"
            })),
            Error::Unauthorized => HttpResponse::Unauthorized().json(json!({
                "message": "Unauthorized"
            })),
            Error::Forbidden => HttpResponse::Forbidden().json(json!({
                "message": "Forbidden"
            })),
            Error::WrongEmailOrPassword => HttpResponse::Unauthorized().json(json!({
                "message": "Wrong email or password"
            })),

            // 5xx Server Errors
            _ => {
                log::error!("Internal server error: {:?}", self);
                HttpResponse::InternalServerError().json(json!({
                    "message": "Internal server error"
                }))
            }
        }
    }
}

#[macro_export]
macro_rules! handle_response {
    ($result:expr) => {{
        // Macro trong Rust không "kế thừa" context use từ nơi nó được định nghĩa, mà phụ thuộc vào context nơi nó được gọi.
        use actix_web::HttpResponse;

        match $result {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(error) => {
                use crate::utils::response_handler::ErrorToHttp;
                error.to_http_response()
            }
        }
    }};
    ($result:expr, $success_status:expr) => {{
        use actix_web::{HttpResponse, http::StatusCode};

        match $result {
            Ok(data) => match $success_status {
                StatusCode::CREATED => HttpResponse::Created().json(data),
                _ => HttpResponse::Ok().json(data),
            },
            Err(error) => {
                use crate::utils::response_handler::ErrorToHttp;
                error.to_http_response()
            }
        }
    }};
}

#[allow(unused)]
pub fn handle_error(error: Error) -> HttpResponse {
    error.to_http_response()
}
