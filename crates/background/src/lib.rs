use apalis::prelude::{Job, Message};
use chrono::DateTime;
use chrono_tz::Tz;
use common_models::ExportItem;
use enums::{MediaLot, MediaSource};
use fitness_models::GithubExercise;
use media_models::{DeployImportJobInput, ProgressUpdateInput, ReviewPostedEvent};
use serde::{Deserialize, Serialize};
use strum::Display;
use uuid::Uuid;

// The background jobs which cannot be throttled.
#[derive(Debug, Deserialize, Serialize, Display)]
pub enum CoreApplicationJob {
    SyncIntegrationsData(String),
    ReviewPosted(ReviewPostedEvent),
    BulkProgressUpdate(String, Vec<ProgressUpdateInput>),
    EntityAddedToCollection(String, Uuid),
}

impl Message for CoreApplicationJob {
    const NAME: &'static str = "apalis::CoreApplicationJob";
}

// The background jobs which can be deployed by the application.
#[derive(Debug, Deserialize, Serialize, Display)]
pub enum ApplicationJob {
    ImportFromExternalSource(String, Box<DeployImportJobInput>),
    ReEvaluateUserWorkouts(String),
    UpdateMetadata(String, bool),
    UpdateGithubExerciseJob(GithubExercise),
    UpdatePerson(String),
    RecalculateCalendarEvents,
    AssociateGroupWithMetadata(MediaLot, MediaSource, String),
    PerformExport(String, Vec<ExportItem>),
    RecalculateUserSummary(String),
    PerformBackgroundTasks,
    UpdateExerciseLibrary,
}

impl Message for ApplicationJob {
    const NAME: &'static str = "apalis::ApplicationJob";
}

// Cron Jobs
pub struct ScheduledJob(pub DateTime<Tz>);

impl From<DateTime<Tz>> for ScheduledJob {
    fn from(value: DateTime<Tz>) -> Self {
        Self(value)
    }
}

impl Job for ScheduledJob {
    const NAME: &'static str = "apalis::ScheduledJob";
}
