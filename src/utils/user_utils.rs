use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::user::{Entity as UserDB, Model as User};

pub async fn find_user_by_id(id: i32, db: &DatabaseConnection) -> User {
    let user = UserDB::find_by_id(id).one(db).await.unwrap();
    user.unwrap()
}
