use sea_orm_migration::prelude::*;

use crate::m20240321_123642_template::Template;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TemplateDownloads::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TemplateDownloads::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TemplateDownloads::TemplateId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TemplateDownloads::Table, TemplateDownloads::Id)
                            .to(Template::Table, Template::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(TemplateDownloads::CreateAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TemplateDownloads::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TemplateDownloads {
    Table,
    Id,
    TemplateId,
    CreateAt,
}
