use crate::router::controller;

use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use serde_json::json;

use sea_orm::{DatabaseConnection};
use serde::Deserialize;

pub async fn get_all_todos(
    Extension(db): Extension<DatabaseConnection>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let result = controller::get_all_todos(db, username).await;
    match result {
        Ok(response) => Json(response),
        Err(err) => Json(json!({
            "error": "Bad Request",
            "message": format!("{}", err),
        })),
    }
}

pub async fn delete_all_todos(
    Extension(db): Extension<DatabaseConnection>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let result = controller::delete_all_todos(db, username).await;
    match result {
        Ok(response) => Json(response),
        Err(err) => Json(json!({
            "error": "Bad Request",
            "message": format!("{}", err)
        })),
    }
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

pub async fn create_todo(
    Extension(db): Extension<DatabaseConnection>,
    Path(username): Path<String>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let result = controller::create_todo(db, username, payload).await;
    match result {
        Ok(response) => Json(response),
        Err(err) => Json(json!({
            "error": "Bad Request",
            "message": format!("{}", err)
        })),
    }
}

pub async fn delete_todo(
    Extension(db): Extension<DatabaseConnection>,
    Path((username, todo_id)): Path<(String, i32)>,
) -> impl IntoResponse {
    let result = controller::delete_todo(db, username, todo_id).await;
    match result {
        Ok(response) => Json(response),
        Err(err) => Json(json!({
            "error": "Bad Request",
            "message": format!("{}", err)
        })),
    }
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

pub async fn update_todo(
    Extension(db): Extension<DatabaseConnection>,
    Path((username, todo_id)): Path<(String, i32)>,
    Json(payload): Json<UpdateTodo>,
) -> impl IntoResponse {
    let result = controller::update_todo(db, username, todo_id, payload).await;
    match result {
        Ok(response) => Json(response),
        Err(err) => Json(json!({
            "error": "Bad Request",
            "message": format!("{}", err)
        })),
    }
}
