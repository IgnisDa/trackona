use std::sync::Arc;

use apalis::{prelude::Storage, sqlite::SqliteStorage};
use async_graphql::{Context, Object, Result};
use base64::{engine::general_purpose, Engine as _};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder,
};

use crate::{
    background::UpdateExerciseJob,
    entities::{exercise, prelude::Exercise},
    file_storage::FileStorageService,
    models::fitness::{Exercise as GithubExercise, ExerciseAttributes},
};

#[derive(Default)]
pub struct ExerciseQuery;

#[Object]
impl ExerciseQuery {
    /// Get all the exercises in the database
    async fn exercises(&self, gql_ctx: &Context<'_>) -> Result<Vec<exercise::Model>> {
        gql_ctx
            .data_unchecked::<Arc<ExerciseService>>()
            .exercises()
            .await
    }
}

#[derive(Default)]
pub struct ExerciseMutation;

#[Object]
impl ExerciseMutation {
    /// Deploy a job to download update the exercise library
    async fn deploy_update_exercise_library_job(&self, gql_ctx: &Context<'_>) -> Result<i32> {
        gql_ctx
            .data_unchecked::<Arc<ExerciseService>>()
            .deploy_update_exercise_library_job()
            .await
    }
}

#[derive(Debug)]
pub struct ExerciseService {
    db: DatabaseConnection,
    file_storage: Arc<FileStorageService>,
    json_url: String,
    image_prefix_url: String,
    update_exercise: SqliteStorage<UpdateExerciseJob>,
}

impl ExerciseService {
    pub fn new(
        db: &DatabaseConnection,
        file_storage: Arc<FileStorageService>,
        json_url: String,
        image_prefix_url: String,
        update_exercise: &SqliteStorage<UpdateExerciseJob>,
    ) -> Self {
        Self {
            db: db.clone(),
            file_storage,
            json_url,
            image_prefix_url,
            update_exercise: update_exercise.clone(),
        }
    }
}

impl ExerciseService {
    async fn get_all_exercises_from_dataset(&self) -> Result<Vec<GithubExercise>> {
        let data: Vec<GithubExercise> = surf::get(&self.json_url)
            .send()
            .await
            .unwrap()
            .body_json()
            .await
            .unwrap();
        Ok(data
            .into_iter()
            .map(|e| GithubExercise {
                attributes: ExerciseAttributes {
                    images: e
                        .attributes
                        .images
                        .into_iter()
                        .map(|i| format!("{}/{}", self.image_prefix_url, i))
                        .collect(),
                    ..e.attributes
                },
                ..e
            })
            .collect())
    }

    async fn exercises(&self) -> Result<Vec<exercise::Model>> {
        let data = Exercise::find()
            .order_by_asc(exercise::Column::Id)
            .all(&self.db)
            .await?;
        Ok(data)
    }

    async fn deploy_update_exercise_library_job(&self) -> Result<i32> {
        let mut storage = self.update_exercise.clone();
        let exercises = self.get_all_exercises_from_dataset().await?;
        let mut job_ids = vec![];
        for exercise in exercises {
            let job = storage.push(UpdateExerciseJob { exercise }).await?;
            job_ids.push(job.to_string());
        }
        Ok(job_ids.len().try_into().unwrap())
    }

    pub async fn update_exercise(&self, ex: GithubExercise) -> Result<()> {
        if Exercise::find()
            .filter(exercise::Column::Identifier.eq(&ex.identifier))
            .one(&self.db)
            .await?
            .is_none()
        {
            let mut images = vec![];
            let mut attributes = ex.attributes.clone();
            for (_idx, image) in ex.attributes.images.into_iter().enumerate() {
                let image_data = surf::get(image)
                    .send()
                    .await
                    .unwrap()
                    .body_bytes()
                    .await
                    .unwrap();
                let image_base64 = general_purpose::URL_SAFE_NO_PAD.encode(image_data);
                images.push(image_base64);
            }
            attributes.images = images;
            let db_exercise = exercise::ActiveModel {
                name: ActiveValue::Set(ex.name),
                identifier: ActiveValue::Set(ex.identifier),
                attributes: ActiveValue::Set(attributes),
                ..Default::default()
            };
            db_exercise.insert(&self.db).await?;
        }
        Ok(())
    }
}
