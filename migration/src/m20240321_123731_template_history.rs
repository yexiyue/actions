use sea_orm_migration::prelude::*;

use crate::{m20220101_000001_create_table::User, m20240321_123642_template::Template};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TemplateHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TemplateHistory::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TemplateHistory::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(TemplateHistory::TemplateId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TemplateHistory::Table, TemplateHistory::Id)
                            .to(Template::Table, Template::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TemplateHistory::Table, TemplateHistory::Id)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(TemplateHistory::CreateAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TemplateHistory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TemplateHistory {
    Table,
    Id,
    UserId,
    TemplateId,
    CreateAt,
}
