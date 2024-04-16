//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use std::sync::Arc;

use async_graphql::{Result, SimpleObject};
use async_trait::async_trait;
use schematic::Schematic;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    file_storage::FileStorageService,
    models::fitness::{WorkoutInformation, WorkoutSummary},
    traits::GraphqlRepresentation,
};

/// A workout that was completed by the user.
#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject, Schematic,
)]
#[sea_orm(table_name = "workout")]
#[graphql(name = "Workout")]
#[schematic(rename = "Workout", rename_all = "snake_case")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[schema(exclude)]
    #[serde(skip)]
    pub repeated_from: Option<String>,
    pub start_time: DateTimeUtc,
    pub end_time: DateTimeUtc,
    #[schema(exclude)]
    #[graphql(skip)]
    #[serde(skip)]
    pub user_id: i32,
    pub summary: WorkoutSummary,
    pub information: WorkoutInformation,
    pub name: String,
    pub comment: Option<String>,
}

#[async_trait]
impl GraphqlRepresentation for Model {
    async fn graphql_repr(self, file_storage_service: &Arc<FileStorageService>) -> Result<Self> {
        let mut cnv_workout = self.clone();
        for image in cnv_workout.information.assets.images.iter_mut() {
            *image = file_storage_service.get_presigned_url(image.clone()).await;
        }
        for video in cnv_workout.information.assets.videos.iter_mut() {
            *video = file_storage_service.get_presigned_url(video.clone()).await;
        }
        for exercise in cnv_workout.information.exercises.iter_mut() {
            for image in exercise.assets.images.iter_mut() {
                *image = file_storage_service.get_presigned_url(image.clone()).await;
            }
            for video in exercise.assets.videos.iter_mut() {
                *video = file_storage_service.get_presigned_url(video.clone()).await;
            }
        }
        Ok(cnv_workout)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::RepeatedFrom",
        to = "Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    RepeatedFrom,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
