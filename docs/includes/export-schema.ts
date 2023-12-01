// Automatically generated by schematic. DO NOT MODIFY!

/* eslint-disable */

/** The actual statistics that were logged in a user measurement. */
export interface UserMeasurementStats {
	abdominalSkinfold: string | null;
	basalMetabolicRate: string | null;
	bicepsCircumference: string | null;
	bodyFat: string | null;
	bodyFatCaliper: string | null;
	bodyMassIndex: string | null;
	boneMass: string | null;
	calories: string | null;
	chestCircumference: string | null;
	chestSkinfold: string | null;
	custom: Record<string, string> | null;
	hipCircumference: string | null;
	leanBodyMass: string | null;
	muscle: string | null;
	neckCircumference: string | null;
	thighCircumference: string | null;
	thighSkinfold: string | null;
	totalBodyWater: string | null;
	totalDailyEnergyExpenditure: string | null;
	visceralFat: string | null;
	waistCircumference: string | null;
	waistToHeightRatio: string | null;
	waistToHipRatio: string | null;
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

export type MetadataLot = 'AudioBook' | 'Anime' | 'Book' | 'Podcast' | 'Manga' | 'Movie' | 'Show' | 'VideoGame' | 'VisualNovel';

/** A user that has commented on a review. */
export interface ReviewCommentUser {
	id: number;
	name: string;
}

/** Comments left in replies to posted reviews. */
export interface ImportOrExportItemReviewComment {
	createdOn: string;
	id: string;
	/** The user ids of all those who liked it. */
	likedBy: number[];
	text: string;
	user: ReviewCommentUser;
}

/** Review data associated to a rating. */
export interface ImportOrExportItemReview {
	/** The date the review was posted. */
	date: string | null;
	/** Whether to mark the review as a spoiler. Defaults to false. */
	spoiler: boolean | null;
	/** Actual text for the review. */
	text: string | null;
}

/** A rating given to an entity. */
export interface ImportOrExportItemRating {
	/** The comments attached to this review. */
	comments: ImportOrExportItemReviewComment[] | null;
	/** If for a podcast, the episode for which this review was for. */
	podcastEpisodeNumber: number | null;
	/** The score of the review. */
	rating: string | null;
	/** Data about the review. */
	review: ImportOrExportItemReview | null;
	/** If for a show, the episode for which this review was for. */
	showEpisodeNumber: number | null;
	/** If for a show, the season for which this review was for. */
	showSeasonNumber: number | null;
}

/** A specific instance when an entity was seen. */
export interface ImportOrExportMediaItemSeen {
	/** The timestamp when finished watching. */
	endedOn: string | null;
	/** If for a podcast, the episode which was seen. */
	podcastEpisodeNumber: number | null;
	/** The progress of media done. If none, it is considered as done. */
	progress: number | null;
	/** If for a show, the episode which was seen. */
	showEpisodeNumber: number | null;
	/** If for a show, the season which was seen. */
	showSeasonNumber: number | null;
	/** The timestamp when started watching. */
	startedOn: string | null;
}

export type MetadataSource = 'Anilist' | 'Audible' | 'Custom' | 'GoogleBooks' | 'Igdb' | 'Itunes' | 'Listennotes' | 'MangaUpdates' | 'Mal' | 'Openlibrary' | 'Tmdb' | 'Vndb';

/** Details about a specific media item that needs to be imported or exported. */
export interface ImportOrExportMediaItem {
	/** The collections this entity was added to. */
	collections: string[];
	/** The provider identifier. For eg: TMDB-ID, Openlibrary ID and so on. */
	identifier: string;
	/** The type of media. */
	lot: MetadataLot;
	/** The review history for the user. */
	reviews: ImportOrExportItemRating[];
	/** The seen history for the user. */
	seenHistory: ImportOrExportMediaItemSeen[];
	/** The source of media. */
	source: MetadataSource;
	/** An string to help identify it in the original source. */
	sourceId: string;
}

/** Details about a specific creator item that needs to be exported. */
export interface ImportOrExportPersonItem {
	/** The collections this entity was added to. */
	collections: string[];
	/** The name of the creator. */
	name: string;
	/** The review history for the user. */
	reviews: ImportOrExportItemRating[];
}

/** The assets that were uploaded for an entity. */
export interface EntityAssets {
	/** The keys of the S3 images. */
	images: string[];
	/** The keys of the S3 videos. */
	videos: string[];
}

export type ExerciseLot = 'Duration' | 'DistanceAndDuration' | 'Reps' | 'RepsAndWeight';

export type SetLot = 'Normal' | 'WarmUp' | 'Drop' | 'Failure';

export type WorkoutSetPersonalBest = 'Weight' | 'OneRm' | 'Volume' | 'Time' | 'Pace' | 'Reps';

/** Details about the statistics of the set performed. */
export interface WorkoutSetStatistic {
	distance: string | null;
	duration: string | null;
	oneRm: string | null;
	reps: number | null;
	weight: string | null;
}

export interface WorkoutSetTotals {
	weight: string | null;
}

/** Details about the set performed. */
export interface WorkoutSetRecord {
	confirmedAt: string | null;
	lot: SetLot;
	personalBests: WorkoutSetPersonalBest[];
	statistic: WorkoutSetStatistic;
	totals: WorkoutSetTotals;
}

/** The totals of a workout and the different bests achieved. */
export interface WorkoutOrExerciseTotals {
	distance: string;
	duration: string;
	/** The number of personal bests achieved. */
	personalBestsAchieved: number;
	reps: number;
	/** The total seconds that were logged in the rest timer. */
	restTime: number;
	weight: string;
}

/** An exercise that has been processed and committed to the database. */
export interface ProcessedExercise {
	assets: EntityAssets;
	lot: ExerciseLot;
	name: string;
	notes: string[];
	restTime: number | null;
	sets: WorkoutSetRecord[];
	total: WorkoutOrExerciseTotals;
}

export type UserUnitSystem = 'metric' | 'imperial';

/** Information about a workout done. */
export interface WorkoutInformation {
	assets: EntityAssets;
	exercises: ProcessedExercise[];
	/**
	 * Each grouped superset of exercises will be in a vector. They will contain
	 * the `exercise.idx`.
	 */
	supersets: number[][];
	unit: UserUnitSystem;
}

/** The summary about an exercise done in a workout. */
export interface WorkoutSummaryExercise {
	bestSet: WorkoutSetRecord;
	id: string;
	lot: ExerciseLot;
	numSets: number;
}

export interface WorkoutSummary {
	exercises: WorkoutSummaryExercise[];
	total: WorkoutOrExerciseTotals;
}

/** A workout that was completed by the user. */
export interface Workout {
	comment: string | null;
	endTime: string;
	id: string;
	information: WorkoutInformation;
	name: string;
	startTime: string;
	summary: WorkoutSummary;
}

/** Complete export of the user. */
export interface ExportAllResponse {
	/** Data about user's measurements. */
	measurements: UserMeasurement[];
	/** Data about user's media. */
	media: ImportOrExportMediaItem[];
	/** Data about user's people. */
	people: ImportOrExportPersonItem[];
	/** Data about user's workouts. */
	workouts: Workout[];
}
