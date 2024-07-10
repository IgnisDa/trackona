//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use database::MetadataToMetadataRelation;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "metadata_to_metadata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub from_metadata_id: String,
    pub relation: MetadataToMetadataRelation,
    pub to_metadata_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::metadata::Entity",
        from = "Column::FromMetadataId",
        to = "super::metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Metadata2,
    #[sea_orm(
        belongs_to = "super::metadata::Entity",
        from = "Column::ToMetadataId",
        to = "super::metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Metadata1,
}

impl ActiveModelBehavior for ActiveModel {}
