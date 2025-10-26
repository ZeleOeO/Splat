use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251025_152142_create_users_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bills::Table)
                    .if_not_exists()
                    .col(pk_auto(Bills::Id))
                    .col(string(Bills::Title))
                    .col(string_null(Bills::Description))
                    .col(integer(Bills::CreatorId))
                    .col(integer(Bills::TotalAmount))
                    .col(string(Bills::Status))
                    .col(string(Bills::Category))
                    .col(date_time(Bills::CreatedAt))
                    .col(date_time(Bills::DueDate))
                    .foreign_key(
                        ForeignKey::create()
                        .from(Bills::Table, Bills::CreatorId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bills::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Bills {
    Table,
    Id,
    Title,
    Description,
    CreatorId,
    TotalAmount,
    CreatedAt,
    DueDate,
    Status,
    Category
}
