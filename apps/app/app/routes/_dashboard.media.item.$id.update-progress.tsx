import {
	Alert,
	Autocomplete,
	Button,
	Checkbox,
	Container,
	Group,
	Select,
	Stack,
	Title,
} from "@mantine/core";
import { DatePickerInput } from "@mantine/dates";
import "@mantine/dates/styles.css";
import { ActionFunctionArgs, LoaderFunctionArgs, json } from "@remix-run/node";
import { Form, useLoaderData } from "@remix-run/react";
import { MediaAdditionalDetailsDocument } from "@ryot/generated/graphql/backend/graphql";
import { formatDateToNaiveDate } from "@ryot/ts-utils";
import { IconAlertCircle } from "@tabler/icons-react";
import { DateTime } from "luxon";
import { useState } from "react";
import invariant from "tiny-invariant";
import { z } from "zod";
import { zx } from "zodix";
import { gqlClient } from "~/lib/api.server";
import { Verb, getVerb } from "~/lib/utilities";
import { ShowAndPodcastSchema, processSubmission } from "~/lib/utils";

const searchParamsSchema = z
	.object({
		title: z.string(),
		isShow: zx.BoolAsString.optional(),
		isPodcast: zx.BoolAsString.optional(),
		onlySeason: zx.BoolAsString.optional(),
		completeShow: zx.BoolAsString.optional(),
		completePodcast: zx.BoolAsString.optional(),
	})
	.merge(ShowAndPodcastSchema);

export type SearchParams = z.infer<typeof searchParamsSchema>;

export const loader = async ({ request, params }: LoaderFunctionArgs) => {
	const query = zx.parseQuery(request, searchParamsSchema);
	const id = params.id ? Number(params.id) : undefined;
	invariant(id, "No ID provided");
	let extraDetails = null;
	if (query.isShow || query.isPodcast) {
		const { mediaDetails } = await gqlClient.request(
			MediaAdditionalDetailsDocument,
			{ metadataId: id },
		);
		extraDetails = mediaDetails;
	}
	return json({ query, id, extraDetails });
};

export const action = async ({ request }: ActionFunctionArgs) => {
	const formData = await request.formData();
	const submission = processSubmission(formData, actionSchema);
	console.log(submission);
	return json({});
};

const actionSchema = z.object({
	metadataId: zx.IntAsString,
	date: z.string().optional(),
	showEpisodeNumber: zx.IntAsString.optional(),
	showSeasonNumber: zx.IntAsString.optional(),
	podcastEpisodeNumber: zx.IntAsString.optional(),
	showSpecifics: z
		.array(
			z.object({
				seasonNumber: zx.IntAsString,
				episodes: z.array(zx.IntAsString),
			}),
		)
		.optional(),
	podcastSpecifics: z
		.array(z.object({ episodeNumber: zx.IntAsString }))
		.optional(),
});

export default function Page() {
	const loaderData = useLoaderData<typeof loader>();
	const [selectedDate, setSelectedDate] = useState<Date | null>(null);

	return (
		<Container size="xs">
			<Form method="post">
				<input hidden name="metadataId" defaultValue={loaderData.id} />
				{loaderData.query.showEpisodeNumber ? (
					<input
						hidden
						name="showEpisodeNumber"
						defaultValue={loaderData.query.showEpisodeNumber.toString()}
					/>
				) : undefined}
				{loaderData.query.showSeasonNumber ? (
					<input
						hidden
						name="showSeasonNumber"
						defaultValue={loaderData.query.showSeasonNumber.toString()}
					/>
				) : undefined}
				{loaderData.query.podcastEpisodeNumber ? (
					<input
						hidden
						name="podcastEpisodeNumber"
						defaultValue={loaderData.query.podcastEpisodeNumber?.toString()}
					/>
				) : undefined}
				{loaderData.extraDetails?.showSpecifics ? (
					<input
						hidden
						name="showSpecifics"
						defaultValue={JSON.stringify(
							loaderData.extraDetails.showSpecifics.seasons.map((s) => ({
								seasonNumber: s.seasonNumber,
								episodes: s.episodes.map((e) => e.episodeNumber),
							})),
						)}
					/>
				) : undefined}
				{loaderData.extraDetails?.podcastSpecifics ? (
					<input
						hidden
						name="podcastSpecifics"
						defaultValue={JSON.stringify(
							loaderData.extraDetails.podcastSpecifics.episodes.map((e) => ({
								episodeNumber: e.number,
							})),
						)}
					/>
				) : undefined}
				<Stack p="sm">
					<Title>{loaderData.query.title}</Title>
					{loaderData.extraDetails?.showSpecifics ? (
						<>
							{loaderData.query.onlySeason || loaderData.query.completeShow ? (
								<Alert color="yellow" icon={<IconAlertCircle size={16} />}>
									{loaderData.query.onlySeason
										? `This will mark all episodes of season ${loaderData.query.showSeasonNumber} as seen`
										: loaderData.query.completeShow
										? "This will mark all episodes for this show as seen"
										: undefined}
								</Alert>
							) : undefined}
							{!loaderData.query.completeShow ? (
								<>
									<Title order={6}>
										Select season
										{loaderData.query.onlySeason ? "" : " and episode"}
									</Title>
									<Select
										label="Season"
										data={loaderData.extraDetails.showSpecifics.seasons.map(
											(s) => ({
												label: `${s.seasonNumber}. ${s.name.toString()}`,
												value: s.seasonNumber.toString(),
											}),
										)}
										defaultValue={loaderData.query.showSeasonNumber?.toString()}
									/>
								</>
							) : undefined}
							{loaderData.query.onlySeason ? (
								<Checkbox label="Mark all seasons before this as seen" />
							) : undefined}
							{!loaderData.query.onlySeason &&
							loaderData.query.showSeasonNumber ? (
								<Select
									label="Episode"
									data={
										loaderData.extraDetails.showSpecifics.seasons
											.find(
												(s) =>
													s.seasonNumber ===
													Number(loaderData.query.showSeasonNumber),
											)
											?.episodes.map((e) => ({
												label: `${e.episodeNumber}. ${e.name.toString()}`,
												value: e.episodeNumber.toString(),
											})) || []
									}
									defaultValue={loaderData.query.showEpisodeNumber?.toString()}
								/>
							) : undefined}
						</>
					) : undefined}
					{loaderData.extraDetails?.podcastSpecifics ? (
						loaderData.query.completePodcast ? (
							<Alert color="yellow" icon={<IconAlertCircle size={16} />}>
								This will mark all episodes for this podcast as seen
							</Alert>
						) : (
							<>
								<Title order={6}>Select episode</Title>
								<Autocomplete
									label="Episode"
									data={loaderData.extraDetails.podcastSpecifics.episodes.map(
										(se) => ({
											label: se.title.toString(),
											value: se.number.toString(),
										}),
									)}
									defaultValue={loaderData.query.podcastEpisodeNumber?.toString()}
								/>
							</>
						)
					) : undefined}
					{loaderData.extraDetails?.lot ? (
						<Title order={6}>
							When did you {getVerb(Verb.Read, loaderData.extraDetails.lot)} it?
						</Title>
					) : undefined}
					<Button
						variant="outline"
						type="submit"
						name="date"
						value={DateTime.now().toISODate() || ""}
					>
						Now
					</Button>
					<Button variant="outline" type="submit">
						I do not remember
					</Button>
					<Group grow>
						<DatePickerInput
							dropdownType="modal"
							maxDate={new Date()}
							onChange={setSelectedDate}
							clearable
						/>
						<Button
							variant="outline"
							disabled={selectedDate === null}
							type="submit"
							name="date"
							value={
								selectedDate ? formatDateToNaiveDate(selectedDate) : undefined
							}
						>
							Custom date
						</Button>
					</Group>
				</Stack>
			</Form>
		</Container>
	);
}
