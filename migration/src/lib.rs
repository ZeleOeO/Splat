pub use sea_orm_migration::prelude::*;

mod m20251025_152142_create_users_table;
mod m20251026_173652_create_bills_table;
mod m20251026_181348_create_user_bills_joined_table;
mod m20251028_093342_create_billee_table;
mod m20251031_101521_create_banks_table;
mod m20251031_173354_create_wallet_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251025_152142_create_users_table::Migration),
            Box::new(m20251026_173652_create_bills_table::Migration),
            Box::new(m20251026_181348_create_user_bills_joined_table::Migration),
            Box::new(m20251028_093342_create_billee_table::Migration),
            Box::new(m20251031_101521_create_banks_table::Migration),
            Box::new(m20251031_173354_create_wallet_table::Migration),
        ]
    }
}
