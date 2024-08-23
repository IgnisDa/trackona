//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "daily_user_activity")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub date: Date,
    pub metadata_review_count: i32,
    pub collection_review_count: i32,
    pub metadata_group_review_count: i32,
    pub person_review_count: i32,
    pub measurement_count: i32,
    pub workout_count: i32,
    pub workout_duration: i32,
    pub audio_book_count: i32,
    pub audio_book_duration: i32,
    pub anime_count: i32,
    pub book_count: i32,
    pub book_pages: i32,
    pub podcast_count: i32,
    pub podcast_duration: i32,
    pub manga_count: i32,
    pub movie_count: i32,
    pub movie_duration: i32,
    pub show_count: i32,
    pub show_duration: i32,
    pub video_game_count: i32,
    pub visual_novel_count: i32,
    pub workout_personal_best: i32,
    pub workout_weight: i32,
    pub workout_reps: i32,
    pub workout_distance: i32,
    pub workout_rest_time: i32,
    pub total_metadata_count: i32,
    pub total_review_count: i32,
    pub total_count: i32,
    pub total_duration: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
