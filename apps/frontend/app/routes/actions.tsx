import { $path } from "@ignisda/remix-routes";
import { ActionFunctionArgs, json, redirect } from "@remix-run/node";
import {
	AddEntityToCollectionDocument,
	CommitMediaDocument,
	CreateMediaReminderDocument,
	CreateReviewCommentDocument,
	DeleteMediaReminderDocument,
	DeleteReviewDocument,
	EntityLot,
	MetadataLot,
	MetadataSource,
	PostReviewDocument,
	RemoveEntityFromCollectionDocument,
	ToggleMediaMonitorDocument,
	Visibility,
} from "@ryot/generated/graphql/backend/graphql";
import invariant from "tiny-invariant";
import { match } from "ts-pattern";
import { z } from "zod";
import { zx } from "zodix";
import { getAuthorizationHeader, gqlClient } from "~/lib/api.server";
import { colorSchemeCookie } from "~/lib/cookies.server";
import { redirectToQueryParam } from "~/lib/generals";
import { createToastHeaders } from "~/lib/toast.server";
import {
	MetadataSpecificsSchema,
	getLogoutCookies,
	processSubmission,
} from "~/lib/utilities.server";

export const loader = async () => redirect($path("/"));

export const action = async ({ request }: ActionFunctionArgs) => {
	const formData = await request.clone().formData();
	const url = new URL(request.url);
	const intent = url.searchParams.get("intent") as string;
	invariant(intent, "No intent provided");
	let redirectTo = (formData.get(redirectToQueryParam) as string) || "/";
	let headers = {};
	let returnData = {};
	await match(intent)
		.with("commitMedia", async () => {
			const submission = processSubmission(formData, commitMediaSchema);
			const { commitMedia } = await gqlClient.request(
				CommitMediaDocument,
				submission,
				await getAuthorizationHeader(request),
			);
			returnData = { commitMedia };
		})
		.with("toggleColorScheme", async () => {
			const currentColorScheme = await colorSchemeCookie.parse(
				request.headers.get("Cookie") || "",
			);
			const newColorScheme = currentColorScheme === "dark" ? "light" : "dark";
			headers = {
				"Set-Cookie": await colorSchemeCookie.serialize(newColorScheme),
			};
		})
		.with("logout", async () => {
			redirectTo = $path("/auth/login");
			headers = { "Set-Cookie": await getLogoutCookies() };
		})
		.with("createReviewComment", async () => {
			const submission = processSubmission(formData, reviewCommentSchema);
			await gqlClient.request(
				CreateReviewCommentDocument,
				{ input: submission },
				await getAuthorizationHeader(request),
			);
			headers = await createToastHeaders({
				message:
					submission.incrementLikes || submission.decrementLikes
						? "Score changed successfully"
						: `Comment ${
								submission.shouldDelete ? "deleted" : "posted"
						  } successfully`,
				type: "success",
			});
		})
		.with("addEntityToCollection", async () => {
			const [submission, input] =
				getChangeCollectionToEntityVariables(formData);
			for (const collectionName of submission.collectionName) {
				await gqlClient.request(
					AddEntityToCollectionDocument,
					{ input: { ...input, collectionName } },
					await getAuthorizationHeader(request),
				);
			}
			headers = await createToastHeaders({
				message: "Media added to collection successfully",
				type: "success",
			});
		})
		.with("removeEntityFromCollection", async () => {
			const [submission, input] =
				getChangeCollectionToEntityVariables(formData);
			for (const collectionName of submission.collectionName) {
				await gqlClient.request(
					RemoveEntityFromCollectionDocument,
					{ input: { ...input, collectionName } },
					await getAuthorizationHeader(request),
				);
			}
		})
		.with("performReviewAction", async () => {
			const submission = processSubmission(formData, reviewSchema);
			if (submission.shouldDelete) {
				invariant(submission.reviewId, "No reviewId provided");
				await gqlClient.request(
					DeleteReviewDocument,
					{ reviewId: submission.reviewId },
					await getAuthorizationHeader(request),
				);
				headers = await createToastHeaders({
					message: "Review deleted successfully",
					type: "success",
				});
			} else {
				await gqlClient.request(
					PostReviewDocument,
					{ input: submission },
					await getAuthorizationHeader(request),
				);
				headers = await createToastHeaders({
					message: "Review submitted successfully",
					type: "success",
				});
			}
		})
		.with("createMediaReminder", async () => {
			const submission = processSubmission(formData, createMediaReminderSchema);
			const { createMediaReminder } = await gqlClient.request(
				CreateMediaReminderDocument,
				{ input: submission },
				await getAuthorizationHeader(request),
			);
			headers = await createToastHeaders({
				type: !createMediaReminder ? "error" : undefined,
				message: !createMediaReminder
					? "Reminder was not created"
					: "Reminder created successfully",
			});
		})
		.with("deleteMediaReminder", async () => {
			const submission = processSubmission(formData, metadataOrPersonIdSchema);
			await gqlClient.request(
				DeleteMediaReminderDocument,
				submission,
				await getAuthorizationHeader(request),
			);
			headers = await createToastHeaders({
				type: "success",
				message: "Reminder deleted successfully",
			});
		})
		.with("toggleMediaMonitor", async () => {
			const submission = processSubmission(formData, metadataOrPersonIdSchema);
			await gqlClient.request(
				ToggleMediaMonitorDocument,
				{ input: submission },
				await getAuthorizationHeader(request),
			);
			headers = await createToastHeaders({
				type: "success",
				message: "Monitor toggled successfully",
			});
		})
		.run();
	if (Object.keys(returnData).length > 0) return json(returnData, { headers });
	return redirect(redirectTo, { headers });
};

const commitMediaSchema = z.object({
	identifier: z.string(),
	lot: z.nativeEnum(MetadataLot),
	source: z.nativeEnum(MetadataSource),
});

const reviewCommentSchema = z.object({
	reviewId: zx.IntAsString,
	commentId: z.string().optional(),
	text: z.string().optional(),
	decrementLikes: zx.BoolAsString.optional(),
	incrementLikes: zx.BoolAsString.optional(),
	shouldDelete: zx.BoolAsString.optional(),
});

const changeCollectionToEntitySchema = z.object({
	collectionName: z.string().transform((v) => v.split(",")),
	entityId: z.string(),
	entityLot: z.nativeEnum(EntityLot),
});

const reviewSchema = z
	.object({
		shouldDelete: zx.BoolAsString.optional(),
		rating: z.string().optional(),
		text: z.string().optional(),
		visibility: z.nativeEnum(Visibility).optional(),
		spoiler: zx.CheckboxAsString.optional(),
		metadataId: zx.IntAsString.optional(),
		metadataGroupId: zx.IntAsString.optional(),
		collectionId: zx.IntAsString.optional(),
		personId: zx.IntAsString.optional(),
		reviewId: zx.IntAsString.optional(),
	})
	.merge(MetadataSpecificsSchema);

const metadataOrPersonIdSchema = z.object({
	metadataId: zx.IntAsString.optional(),
	personId: zx.IntAsString.optional(),
});

const createMediaReminderSchema = z
	.object({ message: z.string(), remindOn: z.string() })
	.merge(metadataOrPersonIdSchema);

const getChangeCollectionToEntityVariables = (formData: FormData) => {
	const submission = processSubmission(
		formData,
		changeCollectionToEntitySchema,
	);
	const metadataId =
		submission.entityLot === EntityLot.Media
			? Number(submission.entityId)
			: undefined;
	const mediaGroupId =
		submission.entityLot === EntityLot.MediaGroup
			? Number(submission.entityId)
			: undefined;
	const personId =
		submission.entityLot === EntityLot.Person
			? Number(submission.entityId)
			: undefined;
	const exerciseId =
		submission.entityLot === EntityLot.Exercise
			? submission.entityId
			: undefined;
	return [
		submission,
		{ metadataId, mediaGroupId, exerciseId, personId },
	] as const;
};
