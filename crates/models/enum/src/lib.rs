use async_graphql::Enum;
use schematic::ConfigEnum;
use sea_orm::{DeriveActiveEnum, EnumIter, FromJsonQueryResult};
use sea_orm_migration::prelude::*;
use serde::{Deserialize, Serialize};
use strum::Display;

/// The different types of media that can be stored.
#[derive(
    Eq,
    Enum,
    Copy,
    Hash,
    Debug,
    Clone,
    Default,
    EnumIter,
    PartialEq,
    Serialize,
    ConfigEnum,
    Deserialize,
    DeriveActiveEnum,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum MediaLot {
    #[default]
    Book,
    Show,
    Movie,
    Anime,
    Manga,
    Music,
    Podcast,
    AudioBook,
    VideoGame,
    VisualNovel,
}

/// The different sources (or providers) from which data can be obtained from.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Deserialize,
    Serialize,
    Enum,
    Default,
    Hash,
    ConfigEnum,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum MediaSource {
    Mal,
    Igdb,
    Tmdb,
    Vndb,
    #[default]
    Custom,
    Itunes,
    Anilist,
    Audible,
    Hardcover,
    Listennotes,
    GoogleBooks,
    Openlibrary,
    MangaUpdates,
    YoutubeMusic,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, Enum,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum UserLot {
    Admin,
    Normal,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, Enum,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum UserNotificationLot {
    Queued,
    Immediate,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Deserialize,
    Serialize,
    Enum,
    Display,
    Default,
    PartialOrd,
    Ord,
    Hash,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum EntityLot {
    #[default]
    Metadata,
    Person,
    MetadataGroup,
    Exercise,
    Collection,
    Workout,
    WorkoutTemplate,
    Review,
    UserMeasurement,
}

// The different possible states of a seen item.
#[derive(
    Eq,
    Enum,
    Copy,
    Debug,
    Clone,
    Display,
    EnumIter,
    PartialEq,
    Serialize,
    Deserialize,
    DeriveActiveEnum,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum SeenState {
    Completed,
    Dropped,
    InProgress,
    OnAHold,
}

#[derive(
    Eq,
    Enum,
    Copy,
    Debug,
    Clone,
    Default,
    EnumIter,
    PartialEq,
    Serialize,
    ConfigEnum,
    Deserialize,
    DeriveActiveEnum,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum Visibility {
    #[default]
    Public,
    Private,
}

#[derive(
    Eq,
    Enum,
    Copy,
    Debug,
    Clone,
    Display,
    EnumIter,
    PartialEq,
    Serialize,
    Deserialize,
    DeriveActiveEnum,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum ImportSource {
    Igdb,
    Imdb,
    Plex,
    Hevy,
    Trakt,
    Movary,
    Anilist,
    Jellyfin,
    OpenScale,
    StrongApp,
    Goodreads,
    Storygraph,
    Myanimelist,
    GenericJson,
    Mediatracker,
    Audiobookshelf,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Enum,
    Copy,
    Deserialize,
    DeriveActiveEnum,
    EnumIter,
    Eq,
    PartialEq,
    Default,
    ConfigEnum,
    Hash,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseMuscle {
    #[default]
    Abdominals,
    Abductors,
    Adductors,
    Biceps,
    Calves,
    Chest,
    Forearms,
    Glutes,
    Hamstrings,
    Lats,
    #[strum(serialize = "lower_back")]
    #[serde(alias = "lower back")]
    LowerBack,
    #[strum(serialize = "middle_back")]
    #[serde(alias = "middle back")]
    MiddleBack,
    Neck,
    Quadriceps,
    Shoulders,
    Traps,
    Triceps,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Enum,
    Copy,
    Deserialize,
    DeriveActiveEnum,
    EnumIter,
    Eq,
    PartialEq,
    Default,
    ConfigEnum,
    Hash,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseForce {
    #[default]
    Pull,
    Push,
    Static,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Enum,
    Copy,
    Deserialize,
    DeriveActiveEnum,
    EnumIter,
    Eq,
    PartialEq,
    Default,
    ConfigEnum,
    Hash,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseLevel {
    #[default]
    Beginner,
    Expert,
    Intermediate,
}

#[derive(
    Eq,
    Hash,
    Enum,
    Copy,
    Debug,
    Clone,
    EnumIter,
    PartialEq,
    Serialize,
    Deserialize,
    DeriveActiveEnum,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseMechanic {
    Compound,
    Isolation,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Enum,
    Copy,
    Deserialize,
    DeriveActiveEnum,
    EnumIter,
    Eq,
    PartialEq,
    Default,
    ConfigEnum,
    Hash,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseEquipment {
    Bands,
    #[default]
    Barbell,
    BodyOnly,
    Cable,
    Dumbbell,
    #[serde(alias = "exercise ball")]
    ExerciseBall,
    #[serde(alias = "e-z curl bar")]
    EZCurlBar,
    #[serde(alias = "foam roll")]
    FoamRoll,
    #[serde(alias = "body only")]
    Kettlebells,
    Machine,
    #[serde(alias = "medicine ball")]
    MedicineBall,
    Other,
}

/// The different types of exercises that can be done.
#[derive(
    Clone,
    Debug,
    Deserialize,
    Serialize,
    DeriveActiveEnum,
    Eq,
    PartialEq,
    Enum,
    Copy,
    EnumIter,
    ConfigEnum,
    Default,
    Hash,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseLot {
    Duration,
    DistanceAndDuration,
    Reps,
    #[default]
    RepsAndWeight,
}

#[derive(
    Eq,
    Enum,
    Copy,
    Hash,
    Debug,
    Clone,
    Default,
    EnumIter,
    Serialize,
    PartialEq,
    Deserialize,
    DeriveActiveEnum,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum ExerciseSource {
    Github,
    #[default]
    Custom,
}

/// The different types of personal bests that can be achieved on a set.
#[derive(
    Eq,
    Enum,
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Serialize,
    ConfigEnum,
    Deserialize,
    FromJsonQueryResult,
)]
#[serde(rename_all = "snake_case")]
pub enum WorkoutSetPersonalBest {
    #[default]
    Weight,
    OneRm,
    Volume,
    Time,
    Pace,
    Reps,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum MetadataToMetadataRelation {
    Suggestion,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Enum,
    PartialEq,
    Eq,
    DeriveActiveEnum,
    EnumIter,
    Serialize,
    Deserialize,
    Hash,
    Display,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[strum(serialize_all = "snake_case")]
pub enum UserToMediaReason {
    // There is at-least one element in the seen history
    Seen,
    // User has watched this media completely (mostly applies to shows, podcasts etc.)
    Finished,
    Reviewed,
    Collection,
    Reminder,
    Owned,
    Monitoring,
    Watchlist,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Enum,
    PartialEq,
    Eq,
    DeriveActiveEnum,
    EnumIter,
    Serialize,
    Deserialize,
    Hash,
    Display,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationLot {
    Yank,
    Sink,
    Push,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Enum,
    PartialEq,
    Eq,
    DeriveActiveEnum,
    EnumIter,
    Serialize,
    Deserialize,
    Hash,
    Display,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationProvider {
    Emby,
    Kodi,
    Komga,
    Radarr,
    Sonarr,
    PlexSink,
    PlexYank,
    GenericJson,
    YoutubeMusic,
    JellyfinPush,
    JellyfinSink,
    Audiobookshelf,
}

#[derive(
    Eq,
    Enum,
    Copy,
    Clone,
    Debug,
    Display,
    EnumIter,
    PartialEq,
    Serialize,
    Deserialize,
    DeriveActiveEnum,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum NotificationPlatformLot {
    Apprise,
    Discord,
    Gotify,
    Ntfy,
    PushBullet,
    PushOver,
    PushSafer,
    Email,
    Telegram,
}
