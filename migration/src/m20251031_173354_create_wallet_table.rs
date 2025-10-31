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
                    .table(Wallet::Table)
                    .if_not_exists()
                    .col(pk_auto(Wallet::Id))
                    .col(integer(Wallet::UserId))
                    .col(string(Wallet::BusinessName))
                    .col(string(Wallet::AccountNumber))
                    .col(string(Wallet::SettlementBank))
                    .col(string(Wallet::AccountName))
                    .col(string(Wallet::Currency))
                    .col(string(Wallet::SubaccountCode))
                    .foreign_key(
                        ForeignKey::create()
                        .from(Wallet::Table, Wallet::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wallet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Wallet {
    Table,
    Id,
    UserId,
    BusinessName,
    AccountNumber,
    SettlementBank,
    AccountName,
    Currency,
    SubaccountCode,
}
