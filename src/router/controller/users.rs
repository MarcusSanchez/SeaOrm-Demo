use sea_orm::{ActiveModelTrait, Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter};
use serde_json::{json, Value};

use entity::{users};
use users::Entity as Users;
use crate::router::common::registered_user;

pub async fn get_user(
    db: DatabaseConnection,
    username: String,
) -> Result<Value, DbErr> {
    let user = Users::find()
        .filter(users::Column::Username.eq(&username))
        .into_json()
        .one(&db).await?;
    let response = match user {
        Some(u) => u,
        None => json!({
            "error": "Bad Request",
            "message": format!("User {} not found", username)
        })
    };

    Ok(response)
}

pub async fn create_user(
    db: DatabaseConnection,
    username: String,
) -> Result<Value, DbErr> {
    if registered_user(&username, &db).await? {
        return Ok(json!({
            "error": "Bad Request",
            "message": format!("User {} already exists", username)
        }));
    }

    let user = users::ActiveModel {
        username: Set(username.clone()),
        ..Default::default()
    };

    let user = user.insert(&db).await?;
    let response = json!({
        "acknowledged": true,
        "message": format!("User {} created", username),
        "user_id": user.id
    });

    Ok(response)
}

pub async fn update_user(
    db: DatabaseConnection,
    old_username: String,
    new_username: String,
) -> Result<Value, DbErr> {
    let user = Users::find()
        .filter(users::Column::Username.eq(&old_username))
        .one(&db)
        .await?;
    let mut user: users::ActiveModel = match user {
        Some(u) => u.into(),
        None => return Ok(json!({
            "error": "Bad Request",
            "message": format!("User {} not found", old_username)
        })),
    };

    user.username = Set(new_username.clone());
    user.update(&db).await?;
    let response = json!({
        "acknowledged": true,
        "message": format!("User {} updated to {}", old_username, new_username)
    });

    Ok(response)
}

pub async fn delete_user(
    db: DatabaseConnection,
    username: String,
) -> Result<Value, DbErr> {
    let user = Users::find()
        .filter(users::Column::Username.eq(&username))
        .one(&db)
        .await?;
    let response = match user {
        Some(user) => {
            user.delete(&db).await?;
            json!({
                "acknowledged": true,
                "message": format!("User {} deleted", username)
            })
        },
        None => json!({
            "error": "Bad Request",
            "message": format!("User {} does not exist", username)
        }),
    };

    Ok(response)
}

pub async fn get_all_users(
    db: DatabaseConnection,
) -> Result<Value, DbErr> {
    let mut users = Users::find().all(&db).await?;
    users.sort_by(|a, b| a.id.cmp(&b.id));

    let response = json!({
        "users": users,
        "count": users.len()
    });
    Ok(response)
}

pub async fn delete_all_users(
    db: DatabaseConnection,
) -> Result<Value, DbErr> {
    let result = Users::delete_many().exec(&db).await?;
    if result.rows_affected == 0 {
        return Ok(json!({
            "error": "Bad Request",
            "message": "No users to deleted"
        }));
    }

    let response = json!({
        "acknowledged": true,
        "message": "All users deleted"
    });
    Ok(response)
}
