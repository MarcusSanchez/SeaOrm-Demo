mod handlers;
mod controller;
mod common;

use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn start() -> Router {
    Router::new()
        /* USERS ROUTES */
        .route("/users", get(handlers::get_all_users))
        .route("/users", delete(handlers::delete_all_users))
        .route("/users/:username", get(handlers::get_user))
        .route("/users/:username", post(handlers::create_user))
        .route("/users/:username", delete(handlers::delete_user))
        .route("/users/:username/:new_username", put(handlers::update_user))

        /* TODOS ROUTES */
        .route("/todos/:username", get(handlers::get_all_todos))
        .route("/todos/:username", delete(handlers::delete_all_todos))
        .route("/todos/:username", post(handlers::create_todo))
        .route("/todos/:username/:todo_id", delete(handlers::delete_todo))
        .route("/todos/:username/:todo_id", put(handlers::update_todo))
}
