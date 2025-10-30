use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

use crate::{entities::{bills::{self, Entity as BillDB, Model as Bill}, user::{Entity as UserEntity}}, errors::error::AppError};

pub async fn find_bill_by_id(id: &str, db: &DatabaseConnection) -> Option<Bill> {
    let bill = BillDB::find().filter(bills::Column::UniqueId.eq(id)).one(db).await.unwrap();
    bill
}

pub async fn find_bills_by_users(user_id: i32, db: &DatabaseConnection) -> Result<Vec<Bill>, AppError> {
    let user_option = UserEntity::find_by_id(user_id).one(db).await?;
    if let Some(user) = user_option {
        let mut bills = BillDB::find().filter(bills::Column::CreatorId.eq(user.id)).all(db).await?;
        let mut bills_user = user.find_related(BillDB).all(db).await.unwrap();
        bills_user = bills_user.iter().filter(|bill| !bills.contains(*bill)).cloned().collect();
        bills_user.append(&mut bills);

       return Ok(bills_user);
    }
    Ok(Vec::<Bill>::new())
}
