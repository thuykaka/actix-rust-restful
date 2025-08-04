use actix_web::{
    HttpResponse,
    error::{Error, InternalError, JsonPayloadError},
};
use serde_json::json;

// handle JSON error
// if not handle, it will return 400 and string body e.g., "Json deserialize error: missing field password at line 3 column 1"
// TODO: Form, Query, Path error_handler
pub fn json_error_handler(err: JsonPayloadError, _: &actix_web::HttpRequest) -> Error {
    let error_response = HttpResponse::BadRequest().json(json!({
        "status_code": 400,
        "message": format!("{}", err)
    }));

    InternalError::from_response(err, error_response).into()
}
