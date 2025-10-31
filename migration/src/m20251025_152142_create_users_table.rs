use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::UserName))
                    .col(string(User::HashedPassword))
                    .col(string_null(User::Email))
                    .col(string(User::FirstName))
                    .col(string(User::LastName))
                    .col(string(User::UniqueId))
                    .col(date_time(User::CreatedAt))
                    .col(integer_null(User::BillsCreated))
                    .col(integer_null(User::BillsJoined))
                    .to_owned(),
            )
            .await;

        let _a = manager
            .create_index(
                Index::create()
                .table(User::Table)
                .name("idx-unique-id-user")
                .col(User::UniqueId)
                .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    UserName,
    FirstName,
    LastName,
    UniqueId,
    HashedPassword,
    Email,
    CreatedAt,
    BillsCreated,
    BillsJoined
}
