//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use chrono::NaiveDate;
use database::{MediaLot, MediaSource};
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::media::{
    AnimeSpecifics, AudioBookSpecifics, BookSpecifics, MangaSpecifics, MetadataFreeCreator,
    MetadataImage, MetadataStateChanges, MetadataVideo, MovieSpecifics, PodcastSpecifics,
    ShowSpecifics, VideoGameSpecifics, VisualNovelSpecifics, WatchProvider,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "metadata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_on: DateTimeUtc,
    pub lot: MediaLot,
    pub last_updated_on: DateTimeUtc,
    pub title: String,
    pub identifier: String,
    pub source: MediaSource,
    pub is_nsfw: Option<bool>,
    pub is_partial: Option<bool>,
    pub description: Option<String>,
    pub original_language: Option<String>,
    pub publish_year: Option<i32>,
    pub publish_date: Option<NaiveDate>,
    pub production_status: Option<String>,
    pub provider_rating: Option<Decimal>,
    #[sea_orm(column_type = "Json")]
    pub images: Option<Vec<MetadataImage>>,
    #[sea_orm(column_type = "Json")]
    pub videos: Option<Vec<MetadataVideo>>,
    #[sea_orm(column_type = "Json")]
    pub free_creators: Option<Vec<MetadataFreeCreator>>,
    #[sea_orm(column_type = "Json")]
    pub watch_providers: Option<Vec<WatchProvider>>,
    pub audio_book_specifics: Option<AudioBookSpecifics>,
    pub book_specifics: Option<BookSpecifics>,
    pub movie_specifics: Option<MovieSpecifics>,
    pub podcast_specifics: Option<PodcastSpecifics>,
    pub show_specifics: Option<ShowSpecifics>,
    pub video_game_specifics: Option<VideoGameSpecifics>,
    pub visual_novel_specifics: Option<VisualNovelSpecifics>,
    pub anime_specifics: Option<AnimeSpecifics>,
    pub manga_specifics: Option<MangaSpecifics>,
    pub state_changes: Option<MetadataStateChanges>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::calendar_event::Entity")]
    CalendarEvent,
    #[sea_orm(has_many = "super::collection_to_entity::Entity")]
    CollectionToEntity,
    #[sea_orm(has_many = "super::metadata_to_genre::Entity")]
    MetadataToGenre,
    #[sea_orm(has_many = "super::metadata_to_metadata_group::Entity")]
    MetadataToMetadataGroup,
    #[sea_orm(has_many = "super::metadata_to_person::Entity")]
    MetadataToPerson,
    #[sea_orm(has_many = "super::review::Entity")]
    Review,
    #[sea_orm(has_many = "super::seen::Entity")]
    Seen,
    #[sea_orm(has_many = "super::user_to_entity::Entity")]
    UserToEntity,
}

impl Related<super::calendar_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CalendarEvent.def()
    }
}

impl Related<super::collection_to_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CollectionToEntity.def()
    }
}

impl Related<super::metadata_to_genre::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataToGenre.def()
    }
}

impl Related<super::metadata_to_metadata_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataToMetadataGroup.def()
    }
}

impl Related<super::metadata_to_person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataToPerson.def()
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

impl Related<super::user_to_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserToEntity.def()
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

impl Related<super::metadata_group::Entity> for Entity {
    fn to() -> RelationDef {
        super::metadata_to_metadata_group::Relation::MetadataGroup.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::metadata_to_metadata_group::Relation::Metadata
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
