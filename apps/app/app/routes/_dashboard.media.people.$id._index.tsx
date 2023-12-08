import { $path } from "@ignisda/remix-routes";
import {
	Anchor,
	Avatar,
	Box,
	Button,
	Container,
	Group,
	SimpleGrid,
	Stack,
	Tabs,
	Text,
	Title,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { LoaderFunctionArgs, MetaFunction, json } from "@remix-run/node";
import { Link, useLoaderData } from "@remix-run/react";
import {
	EntityLot,
	PersonDetailsDocument,
	UserCollectionsListDocument,
	UserPersonDetailsDocument,
} from "@ryot/generated/graphql/backend/graphql";
import {
	IconDeviceTv,
	IconInfoCircle,
	IconMessageCircle2,
	IconPlayerPlay,
	IconUser,
} from "@tabler/icons-react";
import invariant from "tiny-invariant";
import { MediaDetailsLayout } from "~/components/common";
import {
	AddEntityToCollectionModal,
	DisplayCollection,
	MediaScrollArea,
	PartialMetadataDisplay,
	ReviewItemDisplay,
} from "~/components/media";
import { getAuthorizationHeader, gqlClient } from "~/lib/api.server";
import {
	getCoreDetails,
	getUserDetails,
	getUserPreferences,
} from "~/lib/graphql.server";

export const loader = async ({ request, params }: LoaderFunctionArgs) => {
	const personId = params.id ? Number(params.id) : undefined;
	invariant(personId, "No ID provided");
	const [
		coreDetails,
		userPreferences,
		userDetails,
		{ personDetails },
		{ userPersonDetails },
		{ userCollectionsList: collections },
	] = await Promise.all([
		getCoreDetails(),
		getUserPreferences(request),
		getUserDetails(request),
		gqlClient.request(PersonDetailsDocument, { personId }),
		gqlClient.request(
			UserPersonDetailsDocument,
			{ personId },
			await getAuthorizationHeader(request),
		),
		gqlClient.request(
			UserCollectionsListDocument,
			{},
			await getAuthorizationHeader(request),
		),
	]);
	return json({
		personId,
		coreDetails,
		userPreferences,
		userDetails,
		collections,
		userPersonDetails,
		personDetails,
	});
};

export const meta: MetaFunction = ({ data }) => {
	return [
		{
			title: `${
				// biome-ignore lint/suspicious/noExplicitAny:
				(data as any).personDetails.details.name
			} | Ryot`,
		},
	];
};

export default function Page() {
	const loaderData = useLoaderData<typeof loader>();
	const [
		collectionModalOpened,
		{ open: collectionModalOpen, close: collectionModalClose },
	] = useDisclosure(false);

	return (
		<Container>
			<MediaDetailsLayout
				images={loaderData.personDetails.details.displayImages}
				externalLink={{
					source: loaderData.personDetails.details.source,
					href: loaderData.personDetails.sourceUrl,
				}}
			>
				<Title id="creator-title">
					{loaderData.personDetails.details.name}
				</Title>
				<Text c="dimmed" fz={{ base: "sm", lg: "md" }}>
					{[
						`${
							loaderData.personDetails.contents.flatMap((c) => c.items).length
						} media items`,
						loaderData.personDetails.details.birthDate &&
							`Birth: ${loaderData.personDetails.details.birthDate}`,
						loaderData.personDetails.details.deathDate &&
							`Death: ${loaderData.personDetails.details.deathDate}`,
						loaderData.personDetails.details.place &&
							loaderData.personDetails.details.place,
						loaderData.personDetails.details.gender,
					]
						.filter(Boolean)
						.join(" • ")}
					{loaderData.personDetails.details.website ? (
						<>
							{" "}
							•{" "}
							<Anchor
								href={loaderData.personDetails.details.website}
								target="_blank"
								rel="noopener noreferrer"
							>
								Website
							</Anchor>
						</>
					) : undefined}
				</Text>
				{loaderData.userPersonDetails.collections.length > 0 ? (
					<Group id="entity-collections">
						{loaderData.userPersonDetails.collections.map((col) => (
							<DisplayCollection
								col={col}
								entityId={loaderData.personId.toString()}
								entityLot={EntityLot.Person}
								key={col.id}
							/>
						))}
					</Group>
				) : undefined}
				<Tabs defaultValue="media" variant="outline">
					<Tabs.List mb="xs">
						<Tabs.Tab value="media" leftSection={<IconDeviceTv size={16} />}>
							Media
						</Tabs.Tab>
						{loaderData.personDetails.details.description ? (
							<Tabs.Tab
								value="overview"
								leftSection={<IconInfoCircle size={16} />}
							>
								Overview
							</Tabs.Tab>
						) : undefined}
						{loaderData.personDetails.workedOn.length > 0 ? (
							<Tabs.Tab
								value="workedOn"
								leftSection={<IconPlayerPlay size={16} />}
							>
								Worked on
							</Tabs.Tab>
						) : undefined}
						{loaderData.userPersonDetails.reviews.length > 0 ? (
							<Tabs.Tab
								value="reviews"
								leftSection={<IconMessageCircle2 size={16} />}
							>
								Reviews
							</Tabs.Tab>
						) : undefined}
						<Tabs.Tab value="actions" leftSection={<IconUser size={16} />}>
							Actions
						</Tabs.Tab>
					</Tabs.List>
					<Tabs.Panel value="media">
						<MediaScrollArea coreDetails={loaderData.coreDetails}>
							<Stack>
								{loaderData.personDetails.contents.map((role) => (
									<Box key={role.name}>
										<Title order={3} mb="xs" ta="center">
											{role.name}
										</Title>
										<SimpleGrid cols={{ base: 3, md: 4, lg: 5 }}>
											{role.items.map((item) => (
												<Anchor
													key={item.metadataId}
													data-media-id={item.metadataId}
													component={Link}
													to={$path("/media/item/:id", {
														id: item.metadataId || "",
													})}
												>
													<Avatar
														imageProps={{ loading: "lazy" }}
														src={item.image}
														radius="sm"
														h={100}
														w={85}
														mx="auto"
														alt={`${item.title} picture`}
														styles={{ image: { objectPosition: "top" } }}
													/>
													<Text
														c="dimmed"
														size="xs"
														ta="center"
														lineClamp={1}
														mt={4}
													>
														{item.title}
													</Text>
												</Anchor>
											))}
										</SimpleGrid>
									</Box>
								))}
							</Stack>
						</MediaScrollArea>
					</Tabs.Panel>
					{loaderData.personDetails.details.description ? (
						<Tabs.Panel value="overview">
							<MediaScrollArea coreDetails={loaderData.coreDetails}>
								<div
									// biome-ignore lint/security/noDangerouslySetInnerHtml: generated by the backend securely
									dangerouslySetInnerHTML={{
										__html: loaderData.personDetails.details.description,
									}}
								/>
							</MediaScrollArea>
						</Tabs.Panel>
					) : undefined}
					<Tabs.Panel value="workedOn">
						<MediaScrollArea coreDetails={loaderData.coreDetails}>
							<SimpleGrid cols={{ base: 3, md: 4, lg: 5 }}>
								{loaderData.personDetails.workedOn.map((media) => (
									<PartialMetadataDisplay
										key={media.identifier}
										media={media}
									/>
								))}
							</SimpleGrid>
						</MediaScrollArea>
					</Tabs.Panel>
					<Tabs.Panel value="actions">
						<MediaScrollArea coreDetails={loaderData.coreDetails}>
							<SimpleGrid cols={{ base: 1, md: 2 }} spacing="lg">
								<Button
									variant="outline"
									w="100%"
									component={Link}
									to={$path(
										"/media/:id/post-review",
										{ id: loaderData.personId.toString() },
										{
											entityType: "person",
											title: loaderData.personDetails.details.name,
										},
									)}
								>
									Post a review
								</Button>
								<Button variant="outline" onClick={collectionModalOpen}>
									Add to collection
								</Button>
								<AddEntityToCollectionModal
									onClose={collectionModalClose}
									opened={collectionModalOpened}
									entityId={loaderData.personId.toString()}
									entityLot={EntityLot.Person}
									collections={loaderData.collections.map((c) => c.name)}
								/>
							</SimpleGrid>
						</MediaScrollArea>
					</Tabs.Panel>
					<Tabs.Panel value="reviews">
						<MediaScrollArea coreDetails={loaderData.coreDetails}>
							<Stack>
								{loaderData.userPersonDetails.reviews.map((r) => (
									<ReviewItemDisplay
										review={r}
										key={r.id}
										personId={loaderData.personId}
										title={loaderData.personDetails.details.name}
										user={loaderData.userDetails}
										userPreferences={loaderData.userPreferences}
									/>
								))}
							</Stack>
						</MediaScrollArea>
					</Tabs.Panel>
				</Tabs>
			</MediaDetailsLayout>
		</Container>
	);
}
