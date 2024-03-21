use sea_orm_migration::prelude::*;

use crate::{m20220101_000001_create_table::User, m20240321_123757_comments::Comment};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CommentDislikes::Table)
                    .col(
                        ColumnDef::new(CommentDislikes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .if_not_exists()
                    .col(ColumnDef::new(CommentDislikes::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(CommentDislikes::CommentId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CommentDislikes::Table, CommentDislikes::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CommentDislikes::Table, CommentDislikes::UserId)
                            .to(Comment::Table, Comment::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CommentDislikes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CommentDislikes {
    Table,
    Id,
    UserId,
    CommentId,
}
