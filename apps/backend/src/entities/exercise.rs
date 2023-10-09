//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use std::sync::Arc;

use async_graphql::SimpleObject;
use boilermates::boilermates;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    file_storage::FileStorageService,
    migrator::{ExerciseEquipment, ExerciseForce, ExerciseLevel, ExerciseLot, ExerciseMechanic},
    models::fitness::{ExerciseAttributes, ExerciseMuscles},
    utils::get_stored_asset,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "exercise")]
#[graphql(name = "Exercise")]
#[boilermates("ExerciseSearchItem")]
#[boilermates(attr_for(
    "ExerciseSearchItem",
    "#[derive(Clone, Debug, Deserialize, SimpleObject)]"
))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    #[boilermates(not_in("ExerciseSearchItem"))]
    #[sea_orm(unique)]
    pub identifier: String,
    pub lot: ExerciseLot,
    #[boilermates(not_in("ExerciseSearchItem"))]
    pub level: ExerciseLevel,
    #[boilermates(not_in("ExerciseSearchItem"))]
    pub force: Option<ExerciseForce>,
    #[boilermates(not_in("ExerciseSearchItem"))]
    pub mechanic: Option<ExerciseMechanic>,
    #[boilermates(not_in("ExerciseSearchItem"))]
    pub equipment: Option<ExerciseEquipment>,
    pub attributes: ExerciseAttributes,
    #[boilermates(not_in("ExerciseSearchItem"))]
    #[graphql(skip)]
    pub muscles: ExerciseMuscles,
}

impl Model {
    pub async fn graphql_repr(self, file_storage_service: &Arc<FileStorageService>) -> Self {
        let mut converted_exercise = self.clone();
        let mut images = vec![];
        for image in self.attributes.internal_images.iter() {
            images.push(get_stored_asset(image.clone(), file_storage_service).await);
        }
        converted_exercise.attributes.images = images;
        // FIXME: Remove when https://github.com/SeaQL/sea-orm/issues/1517 is fixed.
        converted_exercise.attributes.muscles = self.muscles.0;
        converted_exercise
    }
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
