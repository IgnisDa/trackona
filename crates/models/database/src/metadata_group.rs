//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use async_graphql::SimpleObject;
use async_trait::async_trait;
use boilermates::boilermates;
use enum_models::{MediaLot, MediaSource};
use media_models::MetadataImage;
use nanoid::nanoid;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "metadata_group")]
#[graphql(name = "MetadataGroup")]
#[boilermates("MetadataGroupWithoutId")]
#[boilermates(attr_for("MetadataGroupWithoutId", "#[derive(Clone, Default, Debug)]"))]
pub struct Model {
    #[boilermates(not_in("MetadataGroupWithoutId"))]
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub parts: i32,
    pub lot: MediaLot,
    pub title: String,
    pub identifier: String,
    pub source: MediaSource,
    #[boilermates(not_in("MetadataGroupWithoutId"))]
    pub is_partial: Option<bool>,
    pub source_url: Option<String>,
    #[sea_orm(ignore)]
    pub display_images: Vec<String>,
    pub description: Option<String>,
    #[sea_orm(column_type = "Json")]
    #[graphql(skip)]
    pub images: Option<Vec<MetadataImage>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::collection_to_entity::Entity")]
    CollectionToEntity,
    #[sea_orm(has_many = "super::metadata_group_to_person::Entity")]
    MetadataGroupToPerson,
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

impl Related<super::metadata_group_to_person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataGroupToPerson.def()
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

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.id = ActiveValue::Set(format!("meg_{}", nanoid!(12)));
        }
        Ok(self)
    }
}
