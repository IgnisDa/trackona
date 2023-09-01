//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::migrator::{MetadataLot, MetadataSource};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "partial_metadata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub identifier: String,
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub image: Option<String>,
    pub lot: MetadataLot,
    pub source: MetadataSource,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::metadata_to_partial_metadata::Entity")]
    MetadataToPartialMetadata,
    #[sea_orm(has_many = "super::partial_metadata_to_metadata_group::Entity")]
    PartialMetadataToMetadataGroup,
}

impl Related<super::metadata_to_partial_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataToPartialMetadata.def()
    }
}

impl Related<super::partial_metadata_to_metadata_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PartialMetadataToMetadataGroup.def()
    }
}

impl Related<super::metadata::Entity> for Entity {
    fn to() -> RelationDef {
        super::metadata_to_partial_metadata::Relation::Metadata.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::metadata_to_partial_metadata::Relation::PartialMetadata
                .def()
                .rev(),
        )
    }
}

impl Related<super::metadata_group::Entity> for Entity {
    fn to() -> RelationDef {
        super::partial_metadata_to_metadata_group::Relation::MetadataGroup.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::partial_metadata_to_metadata_group::Relation::PartialMetadata
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
