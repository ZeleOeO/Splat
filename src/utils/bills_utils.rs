use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Related};

use crate::{entities::{bills::{self, Entity as BillDB, Model as Bill}, user::{self, Entity as UserEntity}}, errors::error::AppError};

pub async fn find_bill_by_id(id: i32, db: &DatabaseConnection) -> Bill {
    let bill = BillDB::find_by_id(id).one(db).await.unwrap();
    bill.unwrap()
}

pub async fn find_bills_by_users(user_id: i32, db: &DatabaseConnection) -> Result<Vec<Bill>, AppError> {
    let user_option = UserEntity::find_by_id(user_id).one(db).await?;
    if let Some(user) = user_option {
        let mut bill = BillDB::find().filter(bills::Column::CreatorId.eq(user.id)).all(db).await?;
        let mut bills_user = user.find_related(BillDB).all(db).await.unwrap();
        bills_user.append(&mut bill);

       return Ok(bills_user);
    }
    Ok(Vec::<Bill>::new())
}
