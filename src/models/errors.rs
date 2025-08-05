use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Dùng cho lỗi DB
    #[error("sea_orm::DbErr: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("sea_orm::TransactionError: {0}")]
    Transaction(#[from] sea_orm::TransactionError<sea_orm::DbErr>),

    #[error("serde_json::Error: {0}")]
    SerdeJson(#[from] serde_json::Error),

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

    #[error("Too Many Requests")]
    TooManyRequests,
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
                "statusCode": 401,
                "message": "Unauthorized",
            })),

            Error::UnauthorizedWithMessage(message) => HttpResponse::Unauthorized().json(json!({
                "statusCode": 401,
                "message": message,
            })),

            Error::BadRequest(message) => HttpResponse::BadRequest().json(json!({
                "statusCode": 400,
                "message": message,
            })),

            Error::TooManyRequests => HttpResponse::TooManyRequests().json(json!({
                "statusCode": 429,
                "message": "Too Many Requests",
            })),

            _ => {
                log::error!("Internal server error: {:?}", self);
                HttpResponse::InternalServerError().json(json!({
                    "statusCode": 500,
                    "message": "Internal server error",
                }))
            } // 5xx Server Errors
        }
    }
}
