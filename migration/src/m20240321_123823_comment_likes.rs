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
                    .table(CommentLikes::Table)
                    .col(
                        ColumnDef::new(CommentLikes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .if_not_exists()
                    .col(ColumnDef::new(CommentLikes::UserId).integer().not_null())
                    .col(ColumnDef::new(CommentLikes::CommentId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(CommentLikes::Table, CommentLikes::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CommentLikes::Table, CommentLikes::UserId)
                            .to(Comment::Table, Comment::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CommentLikes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CommentLikes {
    Table,
    Id,
    UserId,
    CommentId,
}
