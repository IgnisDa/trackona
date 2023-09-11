//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};

use crate::{
    entities::prelude::PartialMetadata,
    migrator::{MetadataLot, MetadataSource},
    models::media::{MediaSpecifics, MetadataImages, MetadataVideos},
};

use super::partial_metadata;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "metadata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_on: DateTimeUtc,
    pub lot: MetadataLot,
    pub last_updated_on: DateTimeUtc,
    pub title: String,
    // FIXME: Remove this
    #[sea_orm(indexed)]
    pub identifier: String,
    pub description: Option<String>,
    pub publish_year: Option<i32>,
    pub publish_date: Option<NaiveDate>,
    pub images: MetadataImages,
    pub videos: MetadataVideos,
    pub source: MetadataSource,
    pub specifics: MediaSpecifics,
    pub production_status: String,
    pub provider_rating: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::metadata_to_collection::Entity")]
    MetadataToCollection,
    #[sea_orm(has_many = "super::metadata_to_creator::Entity")]
    MetadataToCreator,
    #[sea_orm(has_many = "super::metadata_to_genre::Entity")]
    MetadataToGenre,
    #[sea_orm(has_many = "super::metadata_to_partial_metadata::Entity")]
    MetadataToPartialMetadata,
    #[sea_orm(has_many = "super::review::Entity")]
    Review,
    #[sea_orm(has_many = "super::seen::Entity")]
    Seen,
    #[sea_orm(has_many = "super::user_to_metadata::Entity")]
    UserToMetadata,
}

impl Related<super::metadata_to_collection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataToCollection.def()
    }
}

impl Related<super::metadata_to_creator::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataToCreator.def()
    }
}

impl Related<super::metadata_to_genre::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataToGenre.def()
    }
}

impl Related<super::metadata_to_partial_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataToPartialMetadata.def()
    }
}

impl Related<super::review::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Review.def()
    }
}

impl Related<super::seen::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Seen.def()
    }
}

impl Related<super::user_to_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserToMetadata.def()
    }
}

impl Related<super::collection::Entity> for Entity {
    fn to() -> RelationDef {
        super::metadata_to_collection::Relation::Collection.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::metadata_to_collection::Relation::Metadata
                .def()
                .rev(),
        )
    }
}

impl Related<super::genre::Entity> for Entity {
    fn to() -> RelationDef {
        super::metadata_to_genre::Relation::Genre.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::metadata_to_genre::Relation::Metadata.def().rev())
    }
}

impl Related<super::partial_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        super::metadata_to_partial_metadata::Relation::PartialMetadata.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::metadata_to_partial_metadata::Relation::Metadata
                .def()
                .rev(),
        )
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_to_metadata::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_to_metadata::Relation::Metadata.def().rev())
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn after_save<C>(model: Model, db: &C, _insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        if let Some(m) = PartialMetadata::find()
            .filter(partial_metadata::Column::Identifier.eq(model.identifier.clone()))
            .filter(partial_metadata::Column::Lot.eq(model.lot))
            .filter(partial_metadata::Column::Source.eq(model.source))
            .one(db)
            .await?
        {
            let mut m: partial_metadata::ActiveModel = m.into();
            m.metadata_id = ActiveValue::Set(Some(model.id));
            m.update(db).await?;
        }
        Ok(model)
    }

    async fn after_delete<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let copied = self.clone();
        if let Some(m) = PartialMetadata::find()
            .filter(partial_metadata::Column::Identifier.eq(copied.identifier.unwrap()))
            .filter(partial_metadata::Column::Lot.eq(copied.lot.unwrap()))
            .filter(partial_metadata::Column::Source.eq(copied.source.unwrap()))
            .one(db)
            .await?
        {
            let mut m: partial_metadata::ActiveModel = m.into();
            m.metadata_id = ActiveValue::Set(None);
            m.update(db).await?;
        }
        Ok(self)
    }
}
