import { useAutoAnimate } from "@formkit/auto-animate/react";
import {
	ActionIcon,
	Alert,
	Box,
	Button,
	Checkbox,
	Container,
	CopyButton,
	Flex,
	Group,
	Modal,
	MultiSelect,
	NumberInput,
	Paper,
	Select,
	Stack,
	Text,
	TextInput,
	Title,
	Tooltip,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { unstable_defineAction, unstable_defineLoader } from "@remix-run/node";
import type { MetaArgs_SingleFetch } from "@remix-run/react";
import { Form, useActionData, useLoaderData } from "@remix-run/react";
import {
	CreateUserIntegrationDocument,
	DeleteUserIntegrationDocument,
	GenerateAuthTokenDocument,
	IntegrationSource,
	UpdateUserIntegrationDocument,
	UserIntegrationsDocument,
	type UserIntegrationsQuery,
} from "@ryot/generated/graphql/backend/graphql";
import { changeCase, processSubmission } from "@ryot/ts-utils";
import {
	IconCheck,
	IconCopy,
	IconEye,
	IconEyeClosed,
	IconPencil,
	IconTrash,
} from "@tabler/icons-react";
import { useState } from "react";
import { namedAction } from "remix-utils/named-action";
import { match } from "ts-pattern";
import { withQuery } from "ufo";
import { z } from "zod";
import { zx } from "zodix";
import { confirmWrapper } from "~/components/confirmation";
import { dayjsLib } from "~/lib/generals";
import { useConfirmSubmit, useUserCollections } from "~/lib/hooks";
import { createToastHeaders, serverGqlService } from "~/lib/utilities.server";

const YANK_INTEGRATIONS = [IntegrationSource.Audiobookshelf];
const PUSH_INTEGRATIONS = [IntegrationSource.Radarr];
const NO_EDITING_ALLOWED = PUSH_INTEGRATIONS;

export const loader = unstable_defineLoader(async ({ request }) => {
	const [{ userIntegrations }] = await Promise.all([
		serverGqlService.authenticatedRequest(
			request,
			UserIntegrationsDocument,
			undefined,
		),
	]);
	return { userIntegrations };
});

export const meta = (_args: MetaArgs_SingleFetch<typeof loader>) => {
	return [{ title: "Integration Settings | Ryot" }];
};

export const action = unstable_defineAction(async ({ request }) => {
	const formData = await request.clone().formData();
	return namedAction(request, {
		delete: async () => {
			const submission = processSubmission(formData, deleteSchema);
			await serverGqlService.authenticatedRequest(
				request,
				DeleteUserIntegrationDocument,
				submission,
			);
			return Response.json(
				{ status: "success", generateAuthToken: false } as const,
				{
					headers: await createToastHeaders({
						type: "success",
						message: "Integration deleted successfully",
					}),
				},
			);
		},
		create: async () => {
			const submission = processSubmission(formData, createSchema);
			await serverGqlService.authenticatedRequest(
				request,
				CreateUserIntegrationDocument,
				{ input: submission },
			);
			return Response.json(
				{ status: "success", generateAuthToken: false } as const,
				{
					headers: await createToastHeaders({
						type: "success",
						message: "Integration created successfully",
					}),
				},
			);
		},
		update: async () => {
			const submission = processSubmission(formData, updateSchema);
			// DEV: Reason for this: https://stackoverflow.com/a/11424089/11667450
			submission.isDisabled = submission.isDisabled === true;
			await serverGqlService.authenticatedRequest(
				request,
				UpdateUserIntegrationDocument,
				{ input: submission },
			);
			return Response.json(
				{ status: "success", generateAuthToken: false } as const,
				{
					headers: await createToastHeaders({
						type: "success",
						message: "Integration updated successfully",
					}),
				},
			);
		},
		generateAuthToken: async () => {
			const { generateAuthToken } = await serverGqlService.authenticatedRequest(
				request,
				GenerateAuthTokenDocument,
				{},
			);
			return Response.json({ status: "success", generateAuthToken } as const);
		},
	});
});

const MINIMUM_PROGRESS = "2";
const MAXIMUM_PROGRESS = "95";

const createSchema = z.object({
	source: z.nativeEnum(IntegrationSource),
	minimumProgress: z.string().optional(),
	maximumProgress: z.string().optional(),
	sourceSpecifics: z
		.object({
			plexUsername: z.string().optional(),
			audiobookshelfBaseUrl: z.string().optional(),
			audiobookshelfToken: z.string().optional(),
		})
		.optional(),
	destinationSpecifics: z
		.object({
			radarrBaseUrl: z.string().optional(),
			radarrApiKey: z.string().optional(),
			radarrProfileId: z.number().optional(),
			radarrRootFolderPath: z.string().optional(),
			radarrSyncCollectionIds: z.string().transform((v) => v.split(",")),
		})
		.optional(),
});

const deleteSchema = z.object({
	integrationId: z.string(),
});

const updateSchema = z.object({
	integrationId: z.string(),
	minimumProgress: z.string().optional(),
	maximumProgress: z.string().optional(),
	isDisabled: zx.CheckboxAsString.optional(),
});

export default function Page() {
	const loaderData = useLoaderData<typeof loader>();
	const actionData = useActionData<typeof action>();
	const [
		createIntegrationModalOpened,
		{
			open: openCreateUserYankIntegrationModal,
			close: closeCreateIntegrationModal,
		},
	] = useDisclosure(false);
	const [updateIntegrationModalData, setUpdateIntegrationModalData] =
		useState<Integration | null>(null);

	return (
		<Container size="xs">
			<Stack>
				<Title>Integration settings</Title>
				{loaderData.userIntegrations.length > 0 ? (
					loaderData.userIntegrations.map((i, idx) => (
						<DisplayIntegration
							integration={i}
							key={`${i.id}-${idx}`}
							setUpdateIntegrationModalData={setUpdateIntegrationModalData}
						/>
					))
				) : (
					<Text>No integrations configured</Text>
				)}
				<Box w="100%">
					<Group justify="space-between">
						<Form
							replace
							method="POST"
							action={withQuery("", { intent: "generateAuthToken" })}
						>
							<Button
								variant="light"
								color="orange"
								radius="md"
								type="submit"
								size="xs"
								fullWidth
							>
								Create API token
							</Button>
						</Form>
						<Button
							size="xs"
							variant="light"
							radius="md"
							onClick={openCreateUserYankIntegrationModal}
						>
							Add new integration
						</Button>
					</Group>
					<CreateIntegrationModal
						createModalOpened={createIntegrationModalOpened}
						closeIntegrationModal={closeCreateIntegrationModal}
					/>
					<UpdateIntegrationModal
						updateIntegrationData={updateIntegrationModalData}
						closeIntegrationModal={() => setUpdateIntegrationModalData(null)}
					/>
				</Box>
				{actionData?.generateAuthToken ? (
					<Alert title="This token will be shown only once" color="yellow">
						<Flex align="center">
							<CopyButton value={actionData.generateAuthToken}>
								{({ copied, copy }) => (
									<Tooltip
										label={copied ? "Copied" : "Copy"}
										withArrow
										position="right"
									>
										<ActionIcon color={copied ? "teal" : "gray"} onClick={copy}>
											{copied ? (
												<IconCheck size={16} />
											) : (
												<IconCopy size={16} />
											)}
										</ActionIcon>
									</Tooltip>
								)}
							</CopyButton>
							<TextInput
								value={actionData.generateAuthToken}
								readOnly
								style={{ flex: 1 }}
								onClick={(e) => e.currentTarget.select()}
							/>
						</Flex>
					</Alert>
				) : null}
			</Stack>
		</Container>
	);
}

type Integration = UserIntegrationsQuery["userIntegrations"][number];

const DisplayIntegration = (props: {
	integration: Integration;
	setUpdateIntegrationModalData: (data: Integration | null) => void;
}) => {
	const [parent] = useAutoAnimate();
	const [integrationInputOpened, { toggle: integrationInputToggle }] =
		useDisclosure(false);
	const submit = useConfirmSubmit();

	const integrationUrl =
		typeof window !== "undefined"
			? `${window.location.origin}/_i/${props.integration.id}`
			: "";

	return (
		<Paper p="xs" withBorder>
			<Stack ref={parent}>
				<Flex align="center" justify="space-between">
					<Box>
						<Group gap={4}>
							<Text size="sm" fw="bold">
								{changeCase(props.integration.source)}
							</Text>
							{props.integration.isDisabled ? (
								<Text size="xs">(Paused)</Text>
							) : null}
						</Group>
						<Text size="xs">
							Created: {dayjsLib(props.integration.createdOn).fromNow()}
						</Text>
						{props.integration.lastTriggeredOn ? (
							<Text size="xs">
								Triggered:{" "}
								{dayjsLib(props.integration.lastTriggeredOn).fromNow()}
							</Text>
						) : null}
					</Box>
					<Group>
						{!YANK_INTEGRATIONS.includes(props.integration.source) ? (
							<ActionIcon color="blue" onClick={integrationInputToggle}>
								{integrationInputOpened ? <IconEyeClosed /> : <IconEye />}
							</ActionIcon>
						) : null}
						{!NO_EDITING_ALLOWED.includes(props.integration.source) ? (
							<ActionIcon
								color="indigo"
								variant="subtle"
								onClick={() =>
									props.setUpdateIntegrationModalData(props.integration)
								}
							>
								<IconPencil />
							</ActionIcon>
						) : null}
						<Form method="POST" action={withQuery("", { intent: "delete" })}>
							<input
								type="hidden"
								name="integrationId"
								defaultValue={props.integration.id}
							/>
							<ActionIcon
								type="submit"
								color="red"
								variant="subtle"
								mt={4}
								onClick={async (e) => {
									const form = e.currentTarget.form;
									e.preventDefault();
									const conf = await confirmWrapper({
										confirmation:
											"Are you sure you want to delete this integration?",
									});
									if (conf && form) submit(form);
								}}
							>
								<IconTrash />
							</ActionIcon>
						</Form>
					</Group>
				</Flex>
				{integrationInputOpened ? (
					<TextInput
						value={integrationUrl}
						readOnly
						onClick={(e) => e.currentTarget.select()}
					/>
				) : null}
			</Stack>
		</Paper>
	);
};

const CreateIntegrationModal = (props: {
	createModalOpened: boolean;
	closeIntegrationModal: () => void;
}) => {
	const collections = useUserCollections();
	const [source, setSource] = useState<IntegrationSource | null>(null);

	return (
		<Modal
			opened={props.createModalOpened}
			onClose={props.closeIntegrationModal}
			centered
			withCloseButton={false}
		>
			<Form
				replace
				method="POST"
				onSubmit={() => props.closeIntegrationModal()}
				action={withQuery("", { intent: "create" })}
			>
				<Stack>
					<Select
						label="Select a source"
						name="source"
						required
						data={Object.values(IntegrationSource).map((is) => ({
							label: changeCase(is),
							value: is,
						}))}
						onChange={(e) => setSource(e as IntegrationSource)}
					/>
					{source && !PUSH_INTEGRATIONS.includes(source) ? (
						<Group wrap="nowrap">
							<NumberInput
								size="xs"
								label="Minimum progress"
								description="Progress will not be synced below this value"
								required
								name="minimumProgress"
								defaultValue={MINIMUM_PROGRESS}
								min={0}
								max={100}
							/>
							<NumberInput
								size="xs"
								label="Maximum progress"
								description="After this value, progress will be marked as completed"
								required
								name="maximumProgress"
								defaultValue={MAXIMUM_PROGRESS}
								min={0}
								max={100}
							/>
						</Group>
					) : null}
					{match(source)
						.with(IntegrationSource.Audiobookshelf, () => (
							<>
								<TextInput
									label="Base Url"
									required
									name="sourceSpecifics.audiobookshelfBaseUrl"
								/>
								<TextInput
									label="Token"
									required
									name="sourceSpecifics.audiobookshelfToken"
								/>
							</>
						))
						.with(IntegrationSource.Plex, () => (
							<>
								<TextInput
									label="Username"
									name="sourceSpecifics.plexUsername"
								/>
							</>
						))
						.with(IntegrationSource.Radarr, () => (
							<>
								<TextInput
									label="Base Url"
									required
									name="destinationSpecifics.radarrBaseUrl"
								/>
								<TextInput
									label="Token"
									required
									name="destinationSpecifics.radarrApiKey"
								/>
								<NumberInput
									label="Profile ID"
									required
									name="destinationSpecifics.radarrProfileId"
									defaultValue={1}
								/>
								<TextInput
									label="Root Folder"
									required
									name="destinationSpecifics.radarrRootFolderPath"
								/>
								<MultiSelect
									label="Collections"
									required
									name="destinationSpecifics.radarrSyncCollectionIds"
									data={collections.map((c) => ({
										label: c.name,
										value: c.id,
									}))}
								/>
							</>
						))
						.otherwise(() => undefined)}
					<Button type="submit">Submit</Button>
				</Stack>
			</Form>
		</Modal>
	);
};

const UpdateIntegrationModal = (props: {
	updateIntegrationData: Integration | null;
	closeIntegrationModal: () => void;
}) => {
	return (
		<Modal
			opened={props.updateIntegrationData !== null}
			onClose={props.closeIntegrationModal}
			centered
			withCloseButton={false}
		>
			<Form
				replace
				method="POST"
				onSubmit={() => props.closeIntegrationModal()}
				action={withQuery("", { intent: "update" })}
			>
				<input
					type="hidden"
					name="integrationId"
					defaultValue={props.updateIntegrationData?.id}
				/>
				<Stack>
					<Group wrap="nowrap">
						<NumberInput
							size="xs"
							label="Minimum progress"
							description="Progress will not be synced below this value"
							name="minimumProgress"
							defaultValue={
								props.updateIntegrationData?.minimumProgress || undefined
							}
						/>
						<NumberInput
							size="xs"
							label="Maximum progress"
							description="After this value, progress will be marked as completed"
							name="maximumProgress"
							defaultValue={
								props.updateIntegrationData?.maximumProgress || undefined
							}
						/>
					</Group>
					<Checkbox
						name="isDisabled"
						label="Pause integration"
						defaultChecked={
							props.updateIntegrationData?.isDisabled || undefined
						}
					/>
					<Button type="submit">Submit</Button>
				</Stack>
			</Form>
		</Modal>
	);
};
