use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20251025_152142_create_users_table::User, m20251026_173652_create_bills_table::Bills};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Billee::Table)
                    .if_not_exists()
                    .col(pk_auto(Billee::Id))
                    .col(string(Billee::Name))
                    .col(integer(Billee::Percentage))
                    .col(double(Billee::AmountDue))
                    .col(double(Billee::AmountPaid))
                    .col(string(Billee::Status))
                    .col(integer_null(Billee::UserId))
                    .col(integer(Billee::BillId))
                    .col(string(Billee::PaymentLink))
                    .foreign_key(
                        ForeignKey::create()
                        .from(Billee::Table, Billee::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                        .from(Billee::Table, Billee::BillId)
                        .to(Bills::Table, Bills::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Billee::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Billee {
    Table,
    Id,
    Name,
    Percentage,
    AmountDue,
    AmountPaid,
    Status,
    UserId,
    BillId,
    PaymentLink
}
