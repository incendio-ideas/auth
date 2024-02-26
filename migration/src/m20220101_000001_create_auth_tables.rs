use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Key::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Key::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Key::HashedPassword).string().not_null())
                    .col(ColumnDef::new(Key::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Key::Primary)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(uuid::Uuid::new_v4().to_string()),
                    )
                    .col(ColumnDef::new(Session::UserId).uuid().not_null())
                    .col(ColumnDef::new(Session::ExpiresAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(uuid::Uuid::new_v4().to_string()),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("user")
                    .from(Key::Table, Key::UserId)
                    .to(User::Table, User::Id)
                    .on_update(ForeignKeyAction::Cascade)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("session")
                    .from(Session::Table, Session::UserId)
                    .to(User::Table, User::Id)
                    .on_update(ForeignKeyAction::Cascade)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Key::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_foreign_key(ForeignKey::drop().name("user").table(Key::Table).to_owned())
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("session")
                    .table(Session::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Key {
    Table,
    Id,
    HashedPassword,
    UserId,
    Primary,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    UserId,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
}
