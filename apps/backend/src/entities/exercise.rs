//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{migrator::ExerciseLot, models::fitness::ExerciseAttributes};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "exercise")]
#[graphql(name = "Exercise")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub identifier: String,
    pub lot: ExerciseLot,
    pub attributes: ExerciseAttributes,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_to_exercise::Entity")]
    UserToExercise,
}

impl Related<super::user_to_exercise::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserToExercise.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_to_exercise::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_to_exercise::Relation::Exercise.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
