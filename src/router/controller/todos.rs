use serde_json::{json, Value};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter, Set};

use entity::{todos};
use todos::Entity as Todos;
use crate::router::{handlers};
use crate::router::common::registered_user;

pub async fn get_all_todos(
    db: DatabaseConnection,
    username: String,
) -> Result<Value, DbErr> {
    if !registered_user(&username, &db).await? {
        return Ok(json!({
            "error": "Bad Request",
            "message": format!("User {} not found", username)
        }));
    }

    let mut todos = Todos::find()
        .filter(todos::Column::Username.eq(&username))
        .all(&db)
        .await?;
    todos.sort_by(|a, b| a.id.cmp(&b.id));

    let response = json!({
        "count": todos.len(),
        "todos": todos
    });

    Ok(response)
}

pub async fn delete_all_todos(
    db: DatabaseConnection,
    username: String,
) -> Result<Value, DbErr> {
    if !registered_user(&username, &db).await? {
        return Ok(json!({
            "error": "Bad Request",
            "message": format!("User {} not found", username)
        }));
    }

    let result = Todos::delete_many()
        .filter(todos::Column::Username.eq(&username))
        .exec(&db)
        .await?;
    let response = match result.rows_affected {
        0 => json!({
            "error": "Bad Request",
            "message": format!("No todos found for user {}", username)
        }),
        _ => json!({
            "acknowledged": true,
            "message": format!("Deleted all todos for user {}", username)
        }),
    };

    Ok(response)
}

pub async fn create_todo(
    db: DatabaseConnection,
    username: String,
    payload: handlers::CreateTodo,
) -> Result<Value, DbErr> {
    if !registered_user(&username, &db).await? {
        return Ok(json!({
            "error": "Bad Request",
            "message": format!("User {} not found", username)
        }));
    }

    let todo = todos::ActiveModel {
        title: Set(payload.title),
        description: Set(payload.description),
        username: Set(username.clone()),
        ..Default::default()
    };

    todo.insert(&db).await?;
    let response = json!({
        "acknowledged": true,
        "message": format!("Created todo for user {}", username)
    });

    Ok(response)
}

pub async fn delete_todo(
    db: DatabaseConnection,
    username: String,
    todo_id: i32,
) -> Result<Value, DbErr> {
    let todo = Todos::find_by_id(todo_id)
        .one(&db)
        .await?;
    let todo = match todo {
        Some(t) => t,
        None => return Ok(json!({
            "error": "Bad Request",
            "message": format!("Todo {} not found for user {}", todo_id, username)
        }))
    };

    let result = todo.delete(&db).await?;
    let response = match result.rows_affected {
        0 => json!({
            "error": "Bad Request",
            "message": format!("Error deleting Todo {} for user {}", todo_id, username)
        }),
        _ => json!({
            "acknowledged": true,
            "message": format!("Deleted todo {} for user {}", todo_id, username)
        }),
    };

    Ok(response)
}

pub async fn update_todo(
    db: DatabaseConnection,
    username: String,
    todo_id: i32,
    payload: handlers::UpdateTodo,
) -> Result<Value, DbErr> {
    if payload.title.is_none() && payload.description.is_none() && payload.completed.is_none() {
        return Ok(json!({
            "error": "Bad Request",
            "message": "No fields to update"
        }));
    }

    let todo = Todos::find_by_id(todo_id)
        .one(&db)
        .await?;
    let mut todo: todos::ActiveModel = match todo {
        Some(t) => t.into(),
        None => return Ok(json!({
            "error": "Bad Request",
            "message": format!("Todo {} not found for user {}", todo_id, username)
        }))
    };

    if let Some(t) = payload.title { todo.title = Set(t); }
    if let Some(d) = payload.description { todo.description = Set(d); }
    if let Some(c) = payload.completed { todo.completed = Set(c); }

    todo.update(&db).await?;
    let response = json!({
        "acknowledged": true,
        "message": format!("Updated todo {} for user {}", todo_id, username),
    });

    Ok(response)
}