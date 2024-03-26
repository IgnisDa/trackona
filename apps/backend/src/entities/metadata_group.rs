//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use async_graphql::SimpleObject;
use boilermates::boilermates;
use database::{MediaSource, MetadataLot};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::media::MetadataImage;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "metadata_group")]
#[graphql(name = "MetadataGroup")]
#[boilermates("MetadataGroupWithoutId")]
pub struct Model {
    #[boilermates(not_in("MetadataGroupWithoutId"))]
    #[sea_orm(primary_key)]
    pub id: i32,
    pub parts: i32,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    #[sea_orm(column_type = "Json")]
    #[graphql(skip)]
    pub images: Vec<MetadataImage>,
    #[sea_orm(ignore)]
    pub display_images: Vec<String>,
    pub lot: MetadataLot,
    pub source: MediaSource,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::collection_to_entity::Entity")]
    CollectionToEntity,
    #[sea_orm(has_many = "super::metadata_to_metadata_group::Entity")]
    MetadataToMetadataGroup,
    #[sea_orm(has_many = "super::review::Entity")]
    Review,
    #[sea_orm(has_many = "super::user_to_entity::Entity")]
    UserToEntity,
}

impl Related<super::collection_to_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CollectionToEntity.def()
    }
}

impl Related<super::metadata_to_metadata_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataToMetadataGroup.def()
    }
}

impl Related<super::review::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Review.def()
    }
}

impl Related<super::user_to_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserToEntity.def()
    }
}

impl Related<super::metadata::Entity> for Entity {
    fn to() -> RelationDef {
        super::metadata_to_metadata_group::Relation::Metadata.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::metadata_to_metadata_group::Relation::MetadataGroup
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
