use actix_web::{
    HttpResponse, Responder, delete, get, post, put, web,
    web::{Data, Json, Path},
};
use sea_orm::EntityTrait;

use crate::{
    models::{app_state::AppState, todo::Todo},
    services::todo_service::TodoService,
};
use entity::t_users;

#[get("")]
pub async fn get_todos(app_state: Data<AppState>) -> impl Responder {
    let users = t_users::Entity::find().all(&app_state.db).await;

    match users {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(e) => {
            log::error!("Error: {}", e);
            HttpResponse::InternalServerError().body("Error")
        }
    }
}

#[get("/{id}")]
pub async fn get_todo_by_id(todo_service: Data<TodoService>, id: Path<String>) -> impl Responder {
    let todo = todo_service.get_todo_by_id(id.to_string());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found."
        })),
    }
}

#[post("")]
pub async fn create_todo(todo_service: Data<TodoService>, new_todo: Json<Todo>) -> impl Responder {
    let result = todo_service.create_todo(new_todo.0);

    Json(result)
}

#[put("/{id}")]
pub async fn update_todo_by_id(
    todo_service: Data<TodoService>,
    id: Path<String>,
    updated_todo: Json<Todo>,
) -> impl Responder {
    let todo = todo_service.update_todo_by_id(id.to_string(), updated_todo.0);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found."
        })),
    }
}

#[delete("/{id}")]
pub async fn delete_todo_by_id(
    todo_service: Data<TodoService>,
    id: Path<String>,
) -> impl Responder {
    let todo = todo_service.delete_todo_by_id(id.to_string());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found."
        })),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .service(create_todo)
            .service(get_todos)
            .service(get_todo_by_id)
            .service(delete_todo_by_id)
            .service(update_todo_by_id),
    );
}
