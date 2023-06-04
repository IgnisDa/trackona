import {
	BookSource,
	MetadataLot,
} from "@ryot/generated/graphql/backend/graphql";
import {
	IconBook,
	IconBrandAppleArcade,
	IconDeviceDesktop,
	IconDeviceTv,
	IconHeadphones,
	IconMicrophone,
} from "@tabler/icons-react";
import { camelCase, startCase } from "lodash";
import slugify from "slugify";
import invariant from "tiny-invariant";
import { match } from "ts-pattern";

/**
 * Generate initials for a given string.
 */
export const getInitials = (name: string) => {
	const rgx = new RegExp(/(\p{L}{1})\p{L}+/, "gu");
	const initials = [...name.matchAll(rgx)] || [];
	const actuals = (
		(initials.shift()?.[1] || "") + (initials.pop()?.[1] || "")
	).toUpperCase();
	return actuals;
};

/**
 * Change case to a presentable format
 */
export const changeCase = (name: string) =>
	startCase(camelCase(name.toLowerCase()));

/**
 * Get the correct name of the lot from a string
 */
export const getLot = (lot: unknown) => {
	if (!lot) return undefined;
	const newLot = (lot as string).toLowerCase();
	return match(newLot)
		.with("books", "book", () => MetadataLot.Book)
		.with("movies", "movie", () => MetadataLot.Movie)
		.with("tv", "show", () => MetadataLot.Show)
		.with("games", "video_game", "video game", () => MetadataLot.VideoGame)
		.with("audio book", "audiobooks", "audio_book", () => MetadataLot.AudioBook)
		.with("podcast", () => MetadataLot.Podcast)
		.otherwise(() => undefined);
};

export enum Verb {
	Read = 0,
}

export const getVerb = (verb: Verb, lot: MetadataLot) => {
	return match(verb)
		.with(Verb.Read, () => {
			return match(lot)
				.with(MetadataLot.Book, () => "read")
				.with(MetadataLot.Movie, MetadataLot.Show, () => "watch")
				.with(
					MetadataLot.AudioBook,
					MetadataLot.VideoGame,
					MetadataLot.Podcast,
					() => "play",
				)
				.otherwise(() => {
					return "";
				});
		})
		.otherwise(() => "");
};

/**
 * Convert a file to its base64 representation.
 */
export const fileToText = (file: File): Promise<string> =>
	new Promise((resolve, reject) => {
		const reader = new FileReader();
		reader.readAsText(file);
		reader.onload = () => resolve(reader.result?.toString() || "");
		reader.onerror = reject;
	});

/**
 * Generate a random color based on a seed.
 * Taken from https://stackoverflow.com/a/8134122/11667450
 */
export const generateColor = (seed: number) => {
	const color = Math.floor(Math.abs(Math.sin(seed) * 16777215));
	let newColor = color.toString(16);
	while (newColor.length < 6) {
		newColor = `0${color}`;
	}
	return `#${newColor}`;
};

/**
 * Convert a string to a number by adding the ascii values of the characters.
 */
export const getStringAsciiValue = (input: string) => {
	let total = 0;
	for (let i = 0; i < input.length; i++) {
		total += input.charCodeAt(i);
	}
	return total;
};

export const getMetadataIcon = (lot: MetadataLot) => {
	return match(lot)
		.with(MetadataLot.Book, () => IconBook)
		.with(MetadataLot.Movie, () => IconDeviceTv)
		.with(MetadataLot.Show, () => IconDeviceDesktop)
		.with(MetadataLot.VideoGame, () => IconBrandAppleArcade)
		.with(MetadataLot.AudioBook, () => IconHeadphones)
		.with(MetadataLot.Podcast, () => IconMicrophone)
		.exhaustive();
};

export const getSourceUrl = (
	lot: MetadataLot,
	identifier: string,
	title: string,
	from?: BookSource,
) => {
	const slug = slugify(title, {
		lower: true,
		strict: true,
	});
	switch (lot) {
		case MetadataLot.AudioBook:
			return `https://www.audible.com/pd/${slug}/${identifier}`;
		case MetadataLot.Book: {
			invariant(from, "from should be defined");
			return match(from)
				.with(
					BookSource.OpenLibrary,
					() => `https://openlibrary.org/works/${identifier}/${slug}`,
				)
				.with(
					BookSource.Goodreads,
					() => `https://www.goodreads.com/book/show/${identifier}-${slug}`,
				)
				.with(BookSource.Custom, () => ``)
				.exhaustive();
		}
		case MetadataLot.Movie:
			return `https://www.themoviedb.org/movie/${identifier}-${slug}`;
		case MetadataLot.Podcast:
			return `https://www.listennotes.com/podcasts/${slug}-${identifier}`;
		case MetadataLot.Show:
			return `https://www.themoviedb.org/tv/${identifier}-${slug}`;
		case MetadataLot.VideoGame:
			return `https://www.igdb.com/games/${slug}`;
	}
};
