use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter};

use entity::{users};
use users::Entity as Users;

pub async fn registered_user(username: &String, db: &DatabaseConnection) -> Result<bool, DbErr> {
    let count = Users::find()
        .filter(users::Column::Username.eq(username))
        .count(db)
        .await?;
    Ok(count > 0)
}