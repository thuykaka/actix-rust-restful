use crate::{
    app_state::AppState, handle_response, middlewares::auth_middleware::auth_middleware, models::*,
    utils::jwt::AuthenticatedUser,
};
use actix_web::{
    Responder, get,
    middleware::from_fn,
    post, put,
    web::{Data, Json, ServiceConfig, scope},
};
use actix_web_validation::Validated;

#[post("/signup")]
async fn sign_up(
    app_state: Data<AppState>,
    Validated(body): Validated<Json<request::SignUpRequest>>,
) -> impl Responder {
    // into_inner để lấy ra giá trị từ Json<T>
    let result = app_state.auth_service.sign_up(body.into_inner()).await;
    handle_response!(result, StatusCode::CREATED)
}

#[post("/signin")]
async fn sign_in(
    app_state: Data<AppState>,
    Validated(body): Validated<Json<request::SignInRequest>>,
    // Validated để validate request body, throw error từ main > app.validator_error_handler(Arc::new(validator_error_handler))
) -> impl Responder {
    let result = app_state.auth_service.authenticate(body.into_inner()).await;
    handle_response!(result)
}

/// Get current user information
#[get("/me")]
async fn me(app_state: Data<AppState>, user: AuthenticatedUser) -> impl Responder {
    let result = app_state.auth_service.me(user.sub.clone()).await;
    handle_response!(result)
}

#[put("/update")]
async fn update(
    app_state: Data<AppState>,
    user: AuthenticatedUser,
    Validated(body): Validated<Json<request::UpdateUserRequest>>,
) -> impl Responder {
    let result = app_state
        .auth_service
        .update(user.sub.clone(), body.into_inner())
        .await;
    handle_response!(result)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth").service(sign_up).service(sign_in).service(
            scope("")
                .wrap(from_fn(auth_middleware))
                .service(me)
                .service(update),
        ),
    );
}
