use actix_web::{Error, HttpRequest, HttpResponse, ResponseError, body, http};
use derive_more::Display;
use serde::Serialize;
use serde_json::json;

#[macro_export]
macro_rules! handle_response {
    // Truyền mỗi Result<T, Error> vào macro này
    ($result:expr) => {{
        // Macro trong Rust không "kế thừa" context use từ nơi nó được định nghĩa, mà phụ thuộc vào context nơi nó được gọi.
        use actix_web::HttpResponse;

        match $result {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(error) => {
                use crate::models::errors::ErrorToHttp;
                error.to_http_response()
            }
        }
    }};
    // Truyền cả Result<T, Error> và status code vào macro này
    ($result:expr, $success_status:expr) => {{
        use actix_web::{HttpResponse, http::StatusCode};

        match $result {
            Ok(data) => match $success_status {
                StatusCode::CREATED => HttpResponse::Created().json(data),
                _ => HttpResponse::Ok().json(data),
            },
            Err(error) => {
                use crate::models::errors::ErrorToHttp;
                error.to_http_response()
            }
        }
    }};
}

#[derive(Debug, Serialize, Display)]
#[display("Validation failed: {message}")]
struct ValidateErrorResponse {
    message: String,
    status_code: u16,
    errors: Vec<String>,
}

impl ResponseError for ValidateErrorResponse {
    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        HttpResponse::build(self.status_code()).json(json!(self))
    }
}

pub fn validator_error_handler(e: ::validator::ValidationErrors, _: &HttpRequest) -> Error {
    let mut errors = Vec::new();
    for (_, field_errors) in e.field_errors() {
        for error in field_errors {
            let error_msg = error.message.as_ref().unwrap_or(&error.code);
            errors.push(error_msg.to_string());
        }
    }
    ValidateErrorResponse {
        message: "Bad Request".to_string(),
        status_code: 400 as u16,
        errors,
    }
    .into()
}
