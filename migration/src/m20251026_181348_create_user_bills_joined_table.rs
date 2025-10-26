use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20251025_152142_create_users_table::User, m20251026_173652_create_bills_table::Bills,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserBillsJoined::Table)
                    .if_not_exists()
                    .col(integer(UserBillsJoined::UserId))
                    .col(integer(UserBillsJoined::BillsJoinedId))
                    .primary_key(
                        Index::create()
                            .col(UserBillsJoined::UserId)
                            .col(UserBillsJoined::BillsJoinedId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserBillsJoined::Table, UserBillsJoined::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserBillsJoined::Table, UserBillsJoined::BillsJoinedId)
                            .to(Bills::Table, Bills::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserBillsJoined::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserBillsJoined {
    Table,
    UserId,
    BillsJoinedId,
}
