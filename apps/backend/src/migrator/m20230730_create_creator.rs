use sea_orm_migration::prelude::*;

use super::Metadata;

pub struct Migration;

#[derive(Iden)]
pub enum MetadataToCreator {
    Table,
    MetadataId,
    CreatorId,
    Role,
}

#[derive(Iden)]
pub enum Creator {
    Table,
    Id,
    Name,
    Image,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230730_create_creator"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Creator::Table)
                    .col(
                        ColumnDef::new(Creator::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Creator::Name).string().not_null())
                    .col(ColumnDef::new(Creator::Image).string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("creator__name__index")
                    .table(Creator::Table)
                    .col(Creator::Name)
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(MetadataToCreator::Table)
                    .col(
                        ColumnDef::new(MetadataToCreator::MetadataId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MetadataToCreator::CreatorId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-media-item_creator")
                            .col(MetadataToCreator::MetadataId)
                            .col(MetadataToCreator::CreatorId),
                    )
                    .col(ColumnDef::new(MetadataToCreator::Role).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-media-item_media-item-creator_id")
                            .from(MetadataToCreator::Table, MetadataToCreator::MetadataId)
                            .to(Metadata::Table, Metadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-creator-item_media-item-creator_id")
                            .from(MetadataToCreator::Table, MetadataToCreator::CreatorId)
                            .to(Creator::Table, Creator::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
