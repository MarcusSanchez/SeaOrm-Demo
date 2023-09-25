use axum::extract::{Path};
use axum::{Extension, Json};
use axum::response::IntoResponse;

use sea_orm::{DatabaseConnection};
use serde_json::json;

use crate::router::controller;

pub async fn get_all_users(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let result = controller::get_all_users(db).await;
    match result {
        Ok(response) => Json(response),
        Err(err) => Json(json!({
            "error": "Bad Request",
            "message": format!("{}", err)
        })),
    }
}

pub async fn delete_all_users(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let result = controller::delete_all_users(db).await;
    match result {
        Ok(response) => Json(response),
        Err(err) => Json(json!({
            "error": "Bad Request",
            "message": format!("{}", err)
        })),
    }
}

pub async fn get_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let result = controller::get_user(db, username).await;
    match result {
        Ok(response) => Json(response),
        Err(err) => Json(json!({
            "error": "Bad Request",
            "message": format!("{}", err)
        })),
    }
}

pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let result = controller::create_user(db, username).await;
    match result {
        Ok(response) => Json(response),
        Err(err) => Json(json!({
            "error": "Bad Request",
            "message": format!("{}", err)
        })),
    }
}

pub async fn update_user(
    Extension(db): Extension<DatabaseConnection>,
    Path((old_username, new_username)): Path<(String, String)>,
) -> impl IntoResponse {
    let result = controller::update_user(db, old_username, new_username).await;
    match result {
        Ok(response) => Json(response),
        Err(err) => Json(json!({
            "error": "Bad Request",
            "message": format!("{}", err)
        })),
    }
}

pub async fn delete_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let result = controller::delete_user(db, username).await;
    match result {
        Ok(response) => Json(response),
        Err(err) => Json(json!({
            "error": "Bad Request",
            "message": format!("{}", err)
        })),
    }
}
