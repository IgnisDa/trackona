//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use async_graphql::SimpleObject;
use media_models::{DailyUserActivityHourCount, DailyUserActivityMetadataCount};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "daily_user_activity")]
#[graphql(name = "DailyUserActivity")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub date: Date,
    #[sea_orm(primary_key, auto_increment = false)]
    #[graphql(skip)]
    pub user_id: String,
    pub total_counts: i64,
    pub review_counts: i64,
    pub workout_counts: i64,
    pub measurement_counts: i64,
    #[sea_orm(column_type = "Json")]
    #[graphql(skip)]
    pub hour_counts: Vec<DailyUserActivityHourCount>,
    #[sea_orm(column_type = "Json")]
    pub metadata_counts: Vec<DailyUserActivityMetadataCount>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
