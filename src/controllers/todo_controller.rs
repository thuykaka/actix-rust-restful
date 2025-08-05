use crate::{
    app_state::AppState, handle_response, middlewares::auth_middleware::auth_middleware, models::*,
    utils::jwt::AuthenticatedUser,
};
use actix_web::{
    Responder, delete, get,
    middleware::from_fn,
    post, put,
    web::{Data, Json, Path, Query, ServiceConfig, scope},
};
use actix_web_validation::Validated;

#[get("")]
async fn get_all_todos(
    app_state: Data<AppState>,
    Validated(params): Validated<Query<request::GetAllTodosRequest>>,
) -> impl Responder {
    let result = app_state
        .todo_service
        .get_all_todos(params.into_inner())
        .await;
    handle_response!(result)
}

#[get("/{id}")]
async fn get_todo_by_id(app_state: Data<AppState>, path: Path<String>) -> impl Responder {
    let result = app_state
        .todo_service
        .get_todo_by_id(path.into_inner())
        .await;
    handle_response!(result)
}

#[post("")]
async fn create_todo(
    app_state: Data<AppState>,
    Validated(body): Validated<Json<request::CreateTodoRequest>>,
    user: AuthenticatedUser,
) -> impl Responder {
    let result = app_state
        .todo_service
        .create_todo(body.into_inner(), user.sub.clone())
        .await;
    handle_response!(result)
}

#[delete("/{id}")]
async fn delete_todo(app_state: Data<AppState>, path: Path<String>) -> impl Responder {
    let result = app_state.todo_service.delete_todo(path.into_inner()).await;
    handle_response!(result)
}

#[put("/{id}")]
async fn update_todo(
    app_state: Data<AppState>,
    path: Path<String>,
    Validated(body): Validated<Json<request::UpdateTodoRequest>>,
) -> impl Responder {
    let result = app_state
        .todo_service
        .update_todo(path.into_inner(), body.into_inner())
        .await;
    handle_response!(result)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/todos")
            .wrap(from_fn(auth_middleware))
            .service(get_all_todos)
            .service(get_todo_by_id)
            .service(create_todo)
            .service(delete_todo)
            .service(update_todo),
    );
}
