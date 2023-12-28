use apalis::prelude::Storage;
use async_graphql::{Context, Error, Object, Result};
use nanoid::nanoid;
use rs_utils::IsFeatureEnabled;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    sync::Arc,
};

use crate::{
    background::ApplicationJob, file_storage::FileStorageService,
    fitness::resolver::ExerciseService, miscellaneous::resolver::MiscellaneousService,
    models::ExportItem, traits::AuthProvider, utils::TEMP_DIR,
};

#[derive(Default)]
pub struct ExporterMutation;

#[Object]
impl ExporterMutation {
    /// Deploy a job to export data for a user.
    async fn deploy_export_job(
        &self,
        gql_ctx: &Context<'_>,
        to_export: Vec<ExportItem>,
    ) -> Result<bool> {
        let service = gql_ctx.data_unchecked::<Arc<ExporterService>>();
        let user_id = service.user_id_from_ctx(gql_ctx).await?;
        service.deploy_export_job(user_id, to_export).await
    }
}

pub struct ExporterService {
    config: Arc<config::AppConfig>,
    file_storage_service: Arc<FileStorageService>,
    media_service: Arc<MiscellaneousService>,
    exercise_service: Arc<ExerciseService>,
}

impl AuthProvider for ExporterService {}

impl ExporterService {
    pub fn new(
        config: Arc<config::AppConfig>,
        file_storage_service: Arc<FileStorageService>,
        media_service: Arc<MiscellaneousService>,
        exercise_service: Arc<ExerciseService>,
    ) -> Self {
        Self {
            config,
            file_storage_service,
            media_service,
            exercise_service,
        }
    }

    async fn deploy_export_job(&self, user_id: i32, to_export: Vec<ExportItem>) -> Result<bool> {
        self.media_service
            .perform_application_job
            .clone()
            .push(ApplicationJob::PerformExport(user_id, to_export))
            .await?;
        Ok(true)
    }

    pub async fn perform_export(&self, user_id: i32, to_export: Vec<ExportItem>) -> Result<bool> {
        if !self.config.file_storage.is_enabled() {
            return Err(Error::new(
                "File storage needs to be enabled to perform an export.",
            ));
        }
        let export_path = PathBuf::from(TEMP_DIR).join(format!("ryot-export-{}.json", nanoid!()));
        let file = File::create(&export_path).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(b"{").unwrap();
        for (idx, export) in to_export.iter().enumerate() {
            writer
                .write_all(format!(r#""{}":["#, export).as_bytes())
                .unwrap();
            match export {
                ExportItem::Media => {
                    self.media_service
                        .export_media(user_id, &mut writer)
                        .await?
                }
                ExportItem::People => {
                    self.media_service
                        .export_people(user_id, &mut writer)
                        .await?
                }
                ExportItem::Measurements => {
                    self.exercise_service
                        .export_measurements(user_id, &mut writer)
                        .await?
                }
                ExportItem::Workouts => {
                    self.exercise_service
                        .export_workouts(user_id, &mut writer)
                        .await?
                }
            };
            writer.write_all(b"]").unwrap();
            if idx != to_export.len() - 1 {
                writer.write_all(b",").unwrap();
            }
        }
        writer.write_all(b"}").unwrap();
        let (_, url) = self
            .file_storage_service
            .get_presigned_put_url(
                export_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                format!("exports/user__{}", user_id),
                false
            )
            .await;
        surf::put(url)
            .body_file(&export_path)
            .await
            .unwrap()
            .await
            .unwrap();
        Ok(true)
    }
}
