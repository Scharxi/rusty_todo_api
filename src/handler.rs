use crate::{
    model::{AppState, QueryOptions, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use actix_web::dev::Response;
use chrono::prelude::*;
use uuid::Uuid;


#[get("api/healthcheck")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build simple rest CRUD api with actix";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };

    HttpResponse::Ok().json(response_json)
}

#[get("/todos")]
pub async fn todo_list_handler(
    opts: web::Query<QueryOptions>,
    data: web::Data<AppState>
) -> impl Responder {
    let todos = data.todo_db.lock().unwrap();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    HttpResponse::Ok().json(json_response)
}

#[post("/todos")]
async fn create_todo_handler(
    mut body: web::Json<Todo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();

    let todo = vec.iter().find(|todo| todo.title == body.title);

    if todo.is_some() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with title `{}` already exists.", body.title)
        };

        return HttpResponse::Conflict().json(error_response);
    }

    let uuid = uuid::Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();
    vec.push(body.into_inner());

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo }
    };

    HttpResponse::Created().json(json_response)
}