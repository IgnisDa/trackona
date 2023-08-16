// Automatically generated by schematic. DO NOT MODIFY!

/* eslint-disable */

export interface AnimeAnilistConfig {

}

export interface AnimeConfig {
	/** Settings related to Anilist (anime). */
	anilist: AnimeAnilistConfig;
}

export interface AudibleConfig {
	/** Settings related to locale for making requests Audible. */
	locale: string;
}

export interface AudioBookConfig {
	/** Settings related to Audible. */
	audible: AudibleConfig;
}

export interface GoogleBooksConfig {

}

export type OpenlibraryCoverImageSize = 'S' | 'M' | 'L';

export interface OpenlibraryConfig {
	/** The image sizes to fetch from Openlibrary. */
	cover_image_size: OpenlibraryCoverImageSize;
}

export interface BookConfig {
	/** Settings related to Google Books. */
	googleBooks: GoogleBooksConfig;
	/** Settings related to Openlibrary. */
	openlibrary: OpenlibraryConfig;
}

export interface DatabaseConfig {
	/**
	 * The directory where user auth tokens will be persisted.
	 * @default '/data'
	 */
	auth_db_path: string;
	/**
	 * The database connection string. Supports SQLite, MySQL and Postgres.
	 * Format described in https://www.sea-ql.org/SeaORM/docs/install-and-config/connection.
	 */
	url: string;
}

export interface ExerciseConfig {

}

export interface FileStorageConfig {
	/**
	 * The access key ID for the S3 compatible file storage. **Required*to
	 * enable file storage.
	 */
	s3_access_key_id: string;
	/** The name of the S3 compatible bucket. **Required*to enable file storage. */
	s3_bucket_name: string;
	/**
	 * The region for the S3 compatible file storage.
	 * @default 'us-east-1'
	 */
	s3_region: string;
	/**
	 * The secret access key for the S3 compatible file storage. **Required**
	 * to enable file storage.
	 */
	s3_secret_access_key: string;
	/** The URL for the S3 compatible file storage. */
	s3_url: string;
}

export interface FrontendConfig {
	/**
	 * The height of the right section of an item's details page in pixels.
	 * @default 300
	 */
	item_details_height: number;
}

export interface IntegrationConfig {
	/** The salt used to hash user IDs. */
	hasher_salt: string;
	/**
	 * The maximum progress limit after which a media is considered to be completed.
	 * @default 95
	 */
	maximum_progress_limit: number;
	/**
	 * The minimum progress limit before which a media is considered to be started.
	 * @default 2
	 */
	minimum_progress_limit: number;
	/**
	 * Sync data from [yank](/docs/guides/integrations.md) based integrations
	 * every `n` hours.
	 * @default 2
	 */
	pull_every: number;
}

export interface MangaAnilistConfig {

}

export interface MangaConfig {
	/** Settings related to Anilist (manga). */
	anilist: MangaAnilistConfig;
}

export interface MediaConfig {

}

export interface MoviesTmdbConfig {
	/** The access token for the TMDB API. */
	access_token: string;
	/** The locale to use for making requests to TMDB API. */
	locale: string;
}

export interface MovieConfig {
	/** Settings related to TMDB (movies). */
	tmdb: MoviesTmdbConfig;
}

export interface MusicBrainzConfig {
	/** Used for changing the user agent if your requests are being rate limited. */
	user_agent: string | null;
}

export interface MusicConfig {
	/** Settings related to Music Brainz. */
	musicBrainz: MusicBrainzConfig;
}

export interface ITunesConfig {
	/** The locale to use for making requests to iTunes API. */
	locale: string;
}

export interface ListenNotesConfig {
	/** The access token for the Listennotes API. */
	api_token: string;
}

export interface PodcastConfig {
	/** Settings related to iTunes. */
	itunes: ITunesConfig;
	/** Settings related to Listennotes. */
	listennotes: ListenNotesConfig;
}

export interface SchedulerConfig {
	/**
	 * The url to the SQLite database where job related data needs to be stored.
	 * @default 'sqlite::memory:'
	 */
	database_url: string;
	/**
	 * The number of jobs to process every 5 seconds when updating metadata in
	 * the background.
	 * @default 5
	 */
	rate_limit_num: number;
	/**
	 * Deploy a job every x hours that performs user cleanup and summary
	 * calculation.
	 * @default 12
	 */
	user_cleanup_every: number;
}

export interface ServerConfig {
	/** The path where the config file will be written once the server boots up. */
	config_dump_path: string;
	/** An array of URLs for CORS. */
	cors_origins: string[];
	/**
	 * Whether default credentials will be populated on the login page of the
	 * instance.
	 */
	default_credentials: boolean;
	/**
	 * This will make auth cookies insecure and should be set to `true` if you
	 * are running the server on `localhost`.
	 * [More information](https://github.com/IgnisDa/ryot/issues/23)
	 */
	insecure_cookie: boolean;
	/**
	 * The hours in which a media can be marked as seen again for a user. This
	 * is used so that the same media can not be used marked as started when
	 * it has been already marked as seen in the last `n` hours.
	 * @default 2
	 */
	progress_update_threshold: number;
	/** This will set SameSite=None on the auth cookies. */
	samesite_none: boolean;
}

export interface ShowsTmdbConfig {
	/** The access token for the TMDB API. */
	access_token: string;
	/** The locale to use for making requests to TMDB API. */
	locale: string;
}

export interface ShowConfig {
	/** Settings related to TMDB (shows). */
	tmdb: ShowsTmdbConfig;
}

export interface UsersConfig {
	/**
	 * Whether users will be allowed to change their password in their profile
	 * settings.
	 * @default true
	 */
	allow_changing_password: boolean;
	/**
	 * Whether users will be allowed to change their preferences in their profile
	 * settings.
	 * @default true
	 */
	allow_changing_preferences: boolean;
	/**
	 * Whether users will be allowed to change their username in their profile
	 * settings.
	 * @default true
	 */
	allow_changing_username: boolean;
	/**
	 * Whether new users will be allowed to sign up to this instance.
	 * @default true
	 */
	allow_registration: boolean;
	/**
	 * Whether users will be allowed to post reviews on this instance.
	 * @default false
	 */
	reviews_disabled: boolean;
	/**
	 * The number of days till login auth token is valid.
	 * @default 90
	 */
	token_valid_for_days: number;
}

export type IgdbImageSize = 't_original';

export interface IgdbConfig {
	/** The image sizes to fetch from IGDB. */
	image_size: IgdbImageSize;
}

export interface TwitchConfig {
	/**
	 * The client ID issues by Twitch. **Required*to enable video games
	 * tracking. [More information](/docs/guides/video-games.md)
	 */
	client_id: string;
	/**
	 * The client secret issued by Twitch. **Required*to enable video games
	 * tracking.
	 */
	client_secret: string;
}

export interface VideoGameConfig {
	/** Settings related to IGDB. */
	igdb: IgdbConfig;
	/** Settings related to Twitch. */
	twitch: TwitchConfig;
}

export interface AppConfig {
	/** Settings related to anime. */
	anime: AnimeConfig;
	/** Settings related to audio books. */
	audio_books: AudioBookConfig;
	/** Settings related to books. */
	books: BookConfig;
	/** The database related settings. */
	database: DatabaseConfig;
	/** Settings related to exercises. */
	exercise: ExerciseConfig;
	/** Settings related to file storage. */
	file_storage: FileStorageConfig;
	/** Settings related to frontend storage. */
	frontend: FrontendConfig;
	/** Settings related to external integrations. */
	integration: IntegrationConfig;
	/** Settings related to manga. */
	manga: MangaConfig;
	/** Settings related to media. */
	media: MediaConfig;
	/** Settings related to movies. */
	movies: MovieConfig;
	/** Settings related to music. */
	music: MusicConfig;
	/** Settings related to podcasts. */
	podcasts: PodcastConfig;
	/** Settings related to scheduler. */
	scheduler: SchedulerConfig;
	/** Settings related to server. */
	server: ServerConfig;
	/** Settings related to shows. */
	shows: ShowConfig;
	/** Settings related to users. */
	users: UsersConfig;
	/** Settings related to video games. */
	video_games: VideoGameConfig;
}
