use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bank::Table)
                    .if_not_exists()
                    .col(pk_auto(Bank::Id))
                    .col(string(Bank::Name))
                    .col(string(Bank::Code))
                    .col(string(Bank::Country))
                    .col(string(Bank::Currency))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bank::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Bank {
    Table,
    Id,
    Name,
    Code,
    Country,
    Currency
}
