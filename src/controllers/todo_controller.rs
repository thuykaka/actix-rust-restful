use actix_web::{
    HttpResponse, Responder, delete, get, post, put, web,
    web::{Data, Json, Path},
};

use log::info;

use crate::{models::todo::Todo, services::todo_service::TodoService};

#[get("")]
pub async fn get_todos(todo_service: Data<TodoService>) -> impl Responder {
    HttpResponse::Ok().json(todo_service.get_todos())
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
    info!("new_todo: {:?}", new_todo);
    let result = todo_service.create_todo(new_todo.0);

    info!("create todo result: {:?}", result);

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
