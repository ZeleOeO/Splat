use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::user::{self, Entity as UserDB, Model as User};

pub async fn find_user_by_id(id: &str, db: &DatabaseConnection) -> User {
    let user = UserDB::find().filter(user::Column::UniqueId.eq(id)).one(db).await.unwrap();
    user.unwrap()
}
