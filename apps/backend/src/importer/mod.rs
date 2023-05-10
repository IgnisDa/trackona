use std::sync::Arc;

use apalis::{prelude::Storage, sqlite::SqliteStorage};
use async_graphql::{Context, Enum, InputObject, Object, Result, SimpleObject};
use sea_orm::{prelude::DateTimeUtc, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

use crate::{
    audio_books::resolver::AudioBooksService,
    background::ImportMedia,
    books::resolver::BooksService,
    entities::media_import_report,
    media::resolver::{MediaService, ProgressUpdate, ProgressUpdateAction},
    migrator::{MediaImportSource, MetadataLot},
    misc::resolver::{MiscService, PostReviewInput},
    movies::resolver::MoviesService,
    shows::resolver::ShowsService,
    utils::user_id_from_ctx,
    video_games::resolver::VideoGamesService,
};

mod media_tracker;

#[derive(Debug, Clone, SimpleObject)]
pub struct ImportItemReview {
    date: DateTimeUtc,
    spoiler: bool,
    text: String,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct ImportItemRating {
    id: String,
    review: Option<ImportItemReview>,
    rating: Option<i32>,
}

#[derive(Debug, InputObject, Serialize, Deserialize, Clone)]
pub struct DeployMediaTrackerImportInput {
    /// The base url where the resource is present at
    api_url: String,
    /// An application token generated by an admin
    api_key: String,
}

#[derive(Debug, SimpleObject)]
pub struct ImportItemSeen {
    id: String,
    ended_on: Option<DateTimeUtc>,
    season_number: Option<i32>,
    episode_number: Option<i32>,
}

#[derive(Debug, SimpleObject)]
pub struct ImportItem {
    source_id: String,
    lot: MetadataLot,
    identifier: String,
    seen_history: Vec<ImportItemSeen>,
    reviews: Vec<ImportItemRating>,
}

#[derive(Debug, Enum, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum ImportFailStep {
    ItemDetailsFromSource,
    ReviewTransformation,
    MediaDetailsFromProvider,
}

#[derive(
    Debug, SimpleObject, FromJsonQueryResult, Serialize, Deserialize, Eq, PartialEq, Clone,
)]
pub struct ImportFailedItem {
    lot: MetadataLot,
    step: ImportFailStep,
    identifier: String,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ImportDetails {
    pub total: usize,
}

#[derive(Debug, SimpleObject)]
pub struct ImportResult {
    media: Vec<ImportItem>,
    failed_items: Vec<ImportFailedItem>,
}

#[derive(
    Debug, SimpleObject, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq, Clone,
)]
pub struct ImportResultResponse {
    pub source: MediaImportSource,
    pub import: ImportDetails,
    pub failed_items: Vec<ImportFailedItem>,
}

#[derive(Default)]
pub struct ImporterQuery;

#[Object]
impl ImporterQuery {
    /// Get all the import jobs deployed by the user
    async fn media_import_reports(
        &self,
        gql_ctx: &Context<'_>,
    ) -> Result<Vec<media_import_report::Model>> {
        let user_id = user_id_from_ctx(gql_ctx).await?;
        gql_ctx
            .data_unchecked::<ImporterService>()
            .media_import_reports(user_id)
            .await
    }
}

#[derive(Default)]
pub struct ImporterMutation;

#[Object]
impl ImporterMutation {
    /// Add job to import data from MediaTracker.
    async fn deploy_media_tracker_import(
        &self,
        gql_ctx: &Context<'_>,
        input: DeployMediaTrackerImportInput,
    ) -> Result<String> {
        let user_id = user_id_from_ctx(gql_ctx).await?;
        gql_ctx
            .data_unchecked::<ImporterService>()
            .deploy_media_tracker_import(user_id, input)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct ImporterService {
    audio_books_service: Arc<AudioBooksService>,
    books_service: Arc<BooksService>,
    media_service: Arc<MediaService>,
    misc_service: Arc<MiscService>,
    movies_service: Arc<MoviesService>,
    shows_service: Arc<ShowsService>,
    video_games_service: Arc<VideoGamesService>,
    import_media: SqliteStorage<ImportMedia>,
}

impl ImporterService {
    pub fn new(
        audio_books_service: &AudioBooksService,
        books_service: &BooksService,
        media_service: &MediaService,
        misc_service: &MiscService,
        movies_service: &MoviesService,
        shows_service: &ShowsService,
        video_games_service: &VideoGamesService,
        import_media: &SqliteStorage<ImportMedia>,
    ) -> Self {
        Self {
            audio_books_service: Arc::new(audio_books_service.clone()),
            books_service: Arc::new(books_service.clone()),
            media_service: Arc::new(media_service.clone()),
            misc_service: Arc::new(misc_service.clone()),
            movies_service: Arc::new(movies_service.clone()),
            shows_service: Arc::new(shows_service.clone()),
            video_games_service: Arc::new(video_games_service.clone()),
            import_media: import_media.clone(),
        }
    }

    pub async fn deploy_goodreads_import(&self, user_id: i32) -> Result<String> {
        todo!("Implement import from good read using the CSV export.");
    }

    pub async fn deploy_media_tracker_import(
        &self,
        user_id: i32,
        input: DeployMediaTrackerImportInput,
    ) -> Result<String> {
        let mut storage = self.import_media.clone();
        let mut input = input.clone();
        input.api_url = input.api_url.trim_end_matches("/").to_owned();
        let job = storage.push(ImportMedia { user_id, input }).await.unwrap();
        Ok(job.to_string())
    }

    pub async fn media_import_reports(
        &self,
        user_id: i32,
    ) -> Result<Vec<media_import_report::Model>> {
        self.misc_service.media_import_reports(user_id).await
    }

    pub async fn media_tracker_import(
        &self,
        user_id: i32,
        input: DeployMediaTrackerImportInput,
    ) -> Result<()> {
        let db_import_job = self
            .misc_service
            .start_import_job(user_id, MediaImportSource::MediaTracker)
            .await?;
        let mut import = media_tracker::import(input).await?;
        for (idx, item) in import.media.iter().enumerate() {
            tracing::trace!(
                "Importing media with identifier = {iden}",
                iden = item.identifier
            );
            let data = match item.lot {
                MetadataLot::AudioBook => {
                    self.audio_books_service
                        .commit_audio_book(&item.identifier)
                        .await
                }
                MetadataLot::Book => self.books_service.commit_book(&item.identifier).await,
                MetadataLot::Movie => self.movies_service.commit_movie(&item.identifier).await,
                MetadataLot::Show => self.shows_service.commit_show(&item.identifier).await,
                MetadataLot::VideoGame => {
                    self.video_games_service
                        .commit_video_game(&item.identifier)
                        .await
                }
            };
            let metadata = match data {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("{e:?}");
                    import.failed_items.push(ImportFailedItem {
                        lot: item.lot,
                        step: ImportFailStep::MediaDetailsFromProvider,
                        identifier: item.source_id.to_owned(),
                    });
                    continue;
                }
            };
            for seen in item.seen_history.iter() {
                self.media_service
                    .progress_update(
                        ProgressUpdate {
                            identifier: Some(seen.id.clone()),
                            metadata_id: metadata.id,
                            progress: None,
                            action: ProgressUpdateAction::InThePast,
                            date: seen.ended_on.map(|d| d.date_naive()),
                            season_number: seen.season_number,
                            episode_number: seen.episode_number,
                        },
                        user_id.clone(),
                    )
                    .await?;
            }
            for review in item.reviews.iter() {
                let text = review.review.clone().map(|r| r.text);
                let spoiler = review.review.clone().map(|r| r.spoiler);
                let date = review.review.clone().map(|r| r.date);
                self.misc_service
                    .post_review(
                        &user_id,
                        PostReviewInput {
                            identifier: Some(review.id.clone()),
                            rating: review.rating.map(Into::into),
                            text,
                            spoiler,
                            date,
                            visibility: None,
                            metadata_id: metadata.id,
                            review_id: None,
                            season_number: None,
                            episode_number: None,
                        },
                    )
                    .await?;
            }
            tracing::trace!(
                "Imported item: {idx}, lot: {lot}, identifier: {iden}, history count: {hist}, reviews count: {rev}",
                idx = idx,
                lot = item.lot,
                iden = item.identifier,
                hist = item.seen_history.len(),
                rev = item.reviews.len()
            );
        }

        tracing::info!(
            "Imported {total} media items from {source}",
            total = import.media.len(),
            source = db_import_job.source
        );
        let details = ImportResultResponse {
            source: db_import_job.source,
            import: ImportDetails {
                total: import.media.len() - import.failed_items.len(),
            },
            failed_items: import.failed_items,
        };
        self.misc_service
            .finish_import_job(db_import_job, details)
            .await?;
        Ok(())
    }
}
