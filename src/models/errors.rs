use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Dùng cho lỗi DB
    #[error("sea_orm::DbErr: {0}")]
    Db(#[from] sea_orm::DbErr),

    // Dùng cho lỗi khi start server, phải có nó thì mới dùng await? được
    #[error("{0}")]
    ServerStartFailed(#[from] std::io::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("{0}")]
    UnauthorizedWithMessage(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
}

// Dùng cho auth_mw
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        self.to_http_response()
    }
}

#[allow(unused_imports)]
pub trait ErrorToHttp {
    fn to_http_response(&self) -> HttpResponse;
}

impl ErrorToHttp for Error {
    fn to_http_response(&self) -> HttpResponse {
        match self {
            Error::Unauthorized => HttpResponse::Unauthorized().json(json!({
                "status_code": 401,
                "message": "Unauthorized",
                "errors": []
            })),

            Error::UnauthorizedWithMessage(message) => HttpResponse::Unauthorized().json(json!({
                "status_code": 401,
                "message": message,
                "errors": []
            })),

            Error::BadRequest(message) => HttpResponse::BadRequest().json(json!({
                "status_code": 400,
                "message": message,
                "errors": []
            })),
            _ => {
                log::error!("Internal server error: {:?}", self);
                HttpResponse::InternalServerError().json(json!({
                    "status_code": 500,
                    "message": "Internal server error",
                    "errors": []
                }))
            } // 5xx Server Errors
        }
    }
}
