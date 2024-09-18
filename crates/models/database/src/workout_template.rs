//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use async_graphql::SimpleObject;
use enums::Visibility;
use fitness_models::{WorkoutInformation, WorkoutSummary};
use schematic::Schematic;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject, Serialize, Deserialize, Schematic,
)]
#[sea_orm(table_name = "workout_template")]
#[graphql(name = "WorkoutTemplate")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub visibility: Visibility,
    pub created_on: DateTimeUtc,
    #[graphql(skip)]
    #[serde(skip)]
    pub user_id: String,
    pub summary: WorkoutSummary,
    pub information: WorkoutInformation,
    pub default_rest_timer: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::collection_to_entity::Entity")]
    CollectionToEntity,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(has_many = "super::workout::Entity")]
    Workout,
}

impl Related<super::collection_to_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CollectionToEntity.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::workout::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workout.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
