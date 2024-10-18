// Automatically generated by schematic. DO NOT MODIFY!

/* eslint-disable */

export interface IdAndNamedObject {
	id: string;
	name: string;
}

/** Comments left in replies to posted reviews. */
export interface ImportOrExportItemReviewComment {
	created_on: string;
	id: string;
	/** The user ids of all those who liked it. */
	liked_by: string[];
	text: string;
	user: IdAndNamedObject;
}

export type Visibility = 'public' | 'private';

/** Review data associated to a rating. */
export interface ImportOrExportItemReview {
	/** The date the review was posted. */
	date: string | null;
	/** Whether to mark the review as a spoiler. Defaults to false. */
	spoiler: boolean | null;
	/** Actual text for the review. */
	text: string | null;
	/**
	 * The visibility set by the user.
	 *
	 * @default 'public'
	 */
	visibility: Visibility | null;
}

/** A rating given to an entity. */
export interface ImportOrExportItemRating {
	/** If for an anime, the episode for which this review was for. */
	anime_episode_number: number | null;
	/** The comments attached to this review. */
	comments: ImportOrExportItemReviewComment[] | null;
	/** If for a manga, the chapter for which this review was for. */
	manga_chapter_number: string | null;
	/** If for a podcast, the episode for which this review was for. */
	podcast_episode_number: number | null;
	/** The score of the review. */
	rating: string | null;
	/** Data about the review. */
	review: ImportOrExportItemReview | null;
	/** If for a show, the episode for which this review was for. */
	show_episode_number: number | null;
	/** If for a show, the season for which this review was for. */
	show_season_number: number | null;
}

/** Details about a specific exercise item that needs to be exported. */
export interface ImportOrExportExerciseItem {
	/** The collections this entity was added to. */
	collections: string[];
	/** The name of the exercise. */
	name: string;
	/** The review history for the user. */
	reviews: ImportOrExportItemRating[];
}

/** The actual statistics that were logged in a user measurement. */
export interface UserMeasurementStats {
	abdominal_skinfold: string | null;
	basal_metabolic_rate: string | null;
	biceps_circumference: string | null;
	body_fat: string | null;
	body_fat_caliper: string | null;
	body_mass_index: string | null;
	bone_mass: string | null;
	calories: string | null;
	chest_circumference: string | null;
	chest_skinfold: string | null;
	custom: Record<string, string> | null;
	hip_circumference: string | null;
	lean_body_mass: string | null;
	muscle: string | null;
	neck_circumference: string | null;
	thigh_circumference: string | null;
	thigh_skinfold: string | null;
	total_body_water: string | null;
	total_daily_energy_expenditure: string | null;
	visceral_fat: string | null;
	waist_circumference: string | null;
	waist_to_height_ratio: string | null;
	waist_to_hip_ratio: string | null;
	weight: string | null;
}

/** An export of a measurement taken at a point in time. */
export interface UserMeasurement {
	/** Any comment associated entered by the user. */
	comment: string | null;
	/** The name given to this measurement by the user. */
	name: string | null;
	/** The contents of the actual measurement. */
	stats: UserMeasurementStats;
	/** The date and time this measurement was made. */
	timestamp: string;
}

/** The different types of media that can be stored. */
export type MediaLot = 'audio_book' | 'anime' | 'book' | 'podcast' | 'manga' | 'movie' | 'show' | 'video_game' | 'visual_novel';

/** A specific instance when an entity was seen. */
export interface ImportOrExportMediaItemSeen {
	/** If for an anime, the episode which was seen. */
	anime_episode_number: number | null;
	/** The timestamp when finished watching. */
	ended_on: string | null;
	/** If for a manga, the chapter which was seen. */
	manga_chapter_number: string | null;
	/** If for a manga, the volume which was seen. */
	manga_volume_number: number | null;
	/** If for a podcast, the episode which was seen. */
	podcast_episode_number: number | null;
	/** The progress of media done. If none, it is considered as done. */
	progress: string | null;
	/** The provider this item was watched on. */
	provider_watched_on: string | null;
	/** If for a show, the episode which was seen. */
	show_episode_number: number | null;
	/** If for a show, the season which was seen. */
	show_season_number: number | null;
	/** The timestamp when started watching. */
	started_on: string | null;
}

/** The different sources (or providers) from which data can be obtained from. */
export type MediaSource = 'anilist' | 'audible' | 'custom' | 'google_books' | 'igdb' | 'itunes' | 'listennotes' | 'manga_updates' | 'mal' | 'openlibrary' | 'tmdb' | 'vndb';

/** Details about a specific media item that needs to be imported or exported. */
export interface ImportOrExportMediaItem {
	/** The collections this entity was added to. */
	collections: string[];
	/** The provider identifier. For eg: TMDB-ID, Openlibrary ID and so on. */
	identifier: string;
	/**
	 * The type of media.
	 *
	 * @default 'book'
	 * @type {'audio_book' | 'anime' | 'book' | 'podcast' | 'manga' | 'movie' | 'show' | 'video_game' | 'visual_novel'}
	 */
	lot: MediaLot;
	/** The review history for the user. */
	reviews: ImportOrExportItemRating[];
	/** The seen history for the user. */
	seen_history: ImportOrExportMediaItemSeen[];
	/**
	 * The source of media.
	 *
	 * @default 'audible'
	 * @type {'anilist' | 'audible' | 'custom' | 'google_books' | 'igdb' | 'itunes' | 'listennotes' | 'manga_updates' | 'mal' | 'openlibrary' | 'tmdb' | 'vndb'}
	 */
	source: MediaSource;
	/** An string to help identify it in the original source. */
	source_id: string;
}

/** Details about a specific media group item that needs to be imported or exported. */
export interface ImportOrExportMediaGroupItem {
	/** The collections this entity was added to. */
	collections: string[];
	/** The provider identifier. For eg: TMDB-ID, Openlibrary ID and so on. */
	identifier: string;
	/**
	 * The type of media.
	 *
	 * @default 'book'
	 * @type {'audio_book' | 'anime' | 'book' | 'podcast' | 'manga' | 'movie' | 'show' | 'video_game' | 'visual_novel'}
	 */
	lot: MediaLot;
	/** The review history for the user. */
	reviews: ImportOrExportItemRating[];
	/**
	 * The source of media.
	 *
	 * @default 'audible'
	 * @type {'anilist' | 'audible' | 'custom' | 'google_books' | 'igdb' | 'itunes' | 'listennotes' | 'manga_updates' | 'mal' | 'openlibrary' | 'tmdb' | 'vndb'}
	 */
	source: MediaSource;
	/** Name of the group. */
	title: string;
}

export interface PersonSourceSpecifics {
	is_anilist_studio: boolean | null;
	is_tmdb_company: boolean | null;
}

/** Details about a specific creator item that needs to be exported. */
export interface ImportOrExportPersonItem {
	/** The collections this entity was added to. */
	collections: string[];
	/** The provider identifier. */
	identifier: string;
	/** The name of the creator. */
	name: string;
	/** The review history for the user. */
	reviews: ImportOrExportItemRating[];
	/**
	 * The source of data.
	 *
	 * @default 'audible'
	 * @type {'anilist' | 'audible' | 'custom' | 'google_books' | 'igdb' | 'itunes' | 'listennotes' | 'manga_updates' | 'mal' | 'openlibrary' | 'tmdb' | 'vndb'}
	 */
	source: MediaSource;
	/** The source specific data. */
	source_specifics: PersonSourceSpecifics | null;
}

/** The assets that were uploaded for an entity. */
export interface EntityAssets {
	/** The keys of the S3 images. */
	images: string[];
	/** The keys of the S3 videos. */
	videos: string[];
}

/** The different types of exercises that can be done. */
export type ExerciseLot = 'duration' | 'distance_and_duration' | 'reps' | 'reps_and_weight';

/** The types of set (mostly characterized by exertion level). */
export type SetLot = 'normal' | 'warm_up' | 'drop' | 'failure';

/** The different types of personal bests that can be achieved on a set. */
export type WorkoutSetPersonalBest = 'weight' | 'one_rm' | 'volume' | 'time' | 'pace' | 'reps';

/** Details about the statistics of the set performed. */
export interface WorkoutSetStatistic {
	distance: string | null;
	duration: string | null;
	one_rm: string | null;
	pace: string | null;
	reps: string | null;
	volume: string | null;
	weight: string | null;
}

export interface WorkoutSetTotals {
	weight: string | null;
}

/** Details about the set performed. */
export interface WorkoutSetRecord {
	actual_rest_time: number | null;
	confirmed_at: string | null;
	/** @type {'normal' | 'warm_up' | 'drop' | 'failure'} */
	lot: SetLot;
	note: string | null;
	personal_bests: WorkoutSetPersonalBest[] | null;
	rest_time: number | null;
	statistic: WorkoutSetStatistic;
	totals: WorkoutSetTotals | null;
}

/** The totals of a workout and the different bests achieved. */
export interface WorkoutOrExerciseTotals {
	distance: string;
	duration: string;
	/** The number of personal bests achieved. */
	personal_bests_achieved: number;
	reps: string;
	/** The total seconds that were logged in the rest timer. */
	rest_time?: number;
	weight: string;
}

/** An exercise that has been processed and committed to the database. */
export interface ProcessedExercise {
	assets: EntityAssets | null;
	identifier: string;
	/** @type {'duration' | 'distance_and_duration' | 'reps' | 'reps_and_weight'} */
	lot: ExerciseLot;
	name: string;
	notes: string[];
	sets: WorkoutSetRecord[];
	total: WorkoutOrExerciseTotals | null;
}

export interface WorkoutSupersetsInformation {
	/** A color that will be displayed on the frontend. */
	color: string;
	/** The identifier of all the exercises which are in the same superset */
	exercises: number[];
	identifier: string;
}

/** Information about a workout done. */
export interface WorkoutInformation {
	assets: EntityAssets | null;
	comment: string | null;
	exercises: ProcessedExercise[];
	supersets: WorkoutSupersetsInformation[];
}

/** The summary about an exercise done in a workout. */
export interface WorkoutSummaryExercise {
	best_set: WorkoutSetRecord | null;
	lot: ExerciseLot | null;
	name: string;
	num_sets: number;
}

export interface WorkoutSummary {
	exercises: WorkoutSummaryExercise[];
	total: WorkoutOrExerciseTotals | null;
}

export interface WorkoutTemplate {
	created_on: string;
	id: string;
	information: WorkoutInformation;
	name: string;
	summary: WorkoutSummary;
	/**
	 * @default 'public'
	 * @type {'public' | 'private'}
	 */
	visibility: Visibility;
}

export interface ImportOrExportWorkoutTemplateItem {
	collections: string[];
	details: WorkoutTemplate;
}

/** A workout that was completed by the user. */
export interface Workout {
	duration: number;
	end_time: string;
	id: string;
	information: WorkoutInformation;
	name: string;
	start_time: string;
	summary: WorkoutSummary;
	template_id: string | null;
}

/** Details about a specific exercise item that needs to be exported. */
export interface ImportOrExportWorkoutItem {
	/** The collections this entity was added to. */
	collections: string[];
	/** The details of the workout. */
	details: Workout;
}

/** Complete export of the user. */
export interface CompleteExport {
	/** Data about user's exercises. */
	exercises: ImportOrExportExerciseItem[] | null;
	/** Data about user's measurements. */
	measurements: UserMeasurement[] | null;
	/** Data about user's media. */
	media: ImportOrExportMediaItem[] | null;
	/** Data about user's media groups. */
	media_groups: ImportOrExportMediaGroupItem[] | null;
	/** Data about user's people. */
	people: ImportOrExportPersonItem[] | null;
	/** Data about user's workout templates. */
	workout_templates: ImportOrExportWorkoutTemplateItem[] | null;
	/** Data about user's workouts. */
	workouts: ImportOrExportWorkoutItem[] | null;
}
