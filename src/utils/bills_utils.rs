use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::bills::{Entity as BillDB, Model as Bill};

pub async fn find_bill_by_id(id: i32, db: &DatabaseConnection) -> Bill {
    let bill = BillDB::find_by_id(id).one(db).await.unwrap();
    bill.unwrap()
}
