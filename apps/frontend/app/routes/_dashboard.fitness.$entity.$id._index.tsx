import { useAutoAnimate } from "@formkit/auto-animate/react";
import { $path } from "@ignisda/remix-routes";
import {
	ActionIcon,
	Anchor,
	Avatar,
	Box,
	Button,
	Container,
	Flex,
	Group,
	Image,
	Menu,
	Modal,
	Paper,
	ScrollArea,
	SimpleGrid,
	Stack,
	Text,
	Title,
} from "@mantine/core";
import { DateTimePicker } from "@mantine/dates";
import "@mantine/dates/styles.css";
import { useDisclosure } from "@mantine/hooks";
import { unstable_defineAction, unstable_defineLoader } from "@remix-run/node";
import type { MetaArgs_SingleFetch } from "@remix-run/react";
import { Form, Link, useLoaderData } from "@remix-run/react";
import {
	DeleteUserWorkoutDocument,
	UpdateUserWorkoutDocument,
	WorkoutDetailsDocument,
	type WorkoutDetailsQuery,
	type WorkoutInformation,
	type WorkoutSummary,
} from "@ryot/generated/graphql/backend/graphql";
import { humanizeDuration } from "@ryot/ts-utils";
import {
	IconBarbell,
	IconClock,
	IconClockEdit,
	IconDotsVertical,
	IconInfoCircle,
	IconRepeat,
	IconRotateClockwise,
	IconRun,
	IconTrash,
	IconTrophy,
	IconWeight,
	IconZzz,
} from "@tabler/icons-react";
import { useQuery } from "@tanstack/react-query";
import { type ReactNode, useState } from "react";
import { namedAction } from "remix-utils/named-action";
import { match } from "ts-pattern";
import { withFragment, withQuery } from "ufo";
import { z } from "zod";
import { zx } from "zodix";
import { confirmWrapper } from "~/components/confirmation";
import {
	DisplaySet,
	displayDistanceWithUnit,
	displayWeightWithUnit,
} from "~/components/fitness";
import { dayjsLib } from "~/lib/generals";
import {
	useConfirmSubmit,
	useGetWorkoutStarter,
	useUserUnitSystem,
} from "~/lib/hooks";
import {
	duplicateOldWorkout,
	getExerciseDetailsQuery,
} from "~/lib/state/fitness";
import {
	createToastHeaders,
	processSubmission,
	redirectWithToast,
	serverGqlService,
} from "~/lib/utilities.server";

enum Entity {
	Workout = "workout",
}

type OtherEntity = { id: string; name: string; doneOn?: string };

type MatchReturn = [
	string,
	string | null,
	string | null,
	WorkoutInformation,
	WorkoutSummary,
	OtherEntity | null,
];

export const loader = unstable_defineLoader(async ({ request, params }) => {
	const { id: entityId, entity } = zx.parseParams(params, {
		id: z.string(),
		entity: z.nativeEnum(Entity),
	});
	const [name, startTime, endTime, information, summary, repeatedWorkout] =
		await match(entity)
			.with(Entity.Workout, async (): Promise<MatchReturn> => {
				const [{ workoutDetails }] = await Promise.all([
					serverGqlService.authenticatedRequest(
						request,
						WorkoutDetailsDocument,
						{ workoutId: entityId },
					),
				]);
				let repeatedWorkout = null;
				if (workoutDetails.repeatedFrom) {
					const { workoutDetails: repeatedWorkoutData } =
						await serverGqlService.authenticatedRequest(
							request,
							WorkoutDetailsDocument,
							{ workoutId: workoutDetails.repeatedFrom },
						);
					repeatedWorkout = {
						id: workoutDetails.repeatedFrom,
						name: repeatedWorkoutData.name,
						doneOn: repeatedWorkoutData.startTime,
					};
				}
				return [
					workoutDetails.name,
					workoutDetails.startTime,
					workoutDetails.endTime,
					workoutDetails.information,
					workoutDetails.summary,
					repeatedWorkout,
				] as const;
			})
			.exhaustive();
	return {
		entityId,
		name,
		entity,
		startTime,
		endTime,
		information,
		summary,
		repeatedWorkout,
	};
});

export const meta = ({ data }: MetaArgs_SingleFetch<typeof loader>) => {
	return [{ title: `${data?.name} | Ryot` }];
};

export const action = unstable_defineAction(async ({ request }) => {
	const formData = await request.clone().formData();
	return namedAction(request, {
		edit: async () => {
			const submission = processSubmission(formData, editWorkoutSchema);
			await serverGqlService.authenticatedRequest(
				request,
				UpdateUserWorkoutDocument,
				{ input: submission },
			);
			return Response.json({ status: "success", submission } as const, {
				headers: await createToastHeaders({
					type: "success",
					message: "Workout edited successfully",
				}),
			});
		},
		delete: async () => {
			const submission = processSubmission(formData, deleteSchema);
			await serverGqlService.authenticatedRequest(
				request,
				DeleteUserWorkoutDocument,
				submission,
			);
			return redirectWithToast($path("/fitness/workouts/list"), {
				type: "success",
				message: "Workout deleted successfully",
			});
		},
	});
});

const deleteSchema = z.object({ workoutId: z.string() });

const editWorkoutSchema = z.object({
	startTime: z.string(),
	endTime: z.string(),
	id: z.string(),
});

export default function Page() {
	const loaderData = useLoaderData<typeof loader>();
	const submit = useConfirmSubmit();
	const unitSystem = useUserUnitSystem();
	const [
		adjustTimeModalOpened,
		{ open: adjustTimeModalOpen, close: adjustTimeModalClose },
	] = useDisclosure(false);
	const [isWorkoutLoading, setIsWorkoutLoading] = useState(false);
	const startWorkout = useGetWorkoutStarter();

	return (
		<>
			{loaderData.startTime && loaderData.endTime ? (
				<Modal
					opened={adjustTimeModalOpened}
					onClose={adjustTimeModalClose}
					withCloseButton={false}
					centered
				>
					<Form
						replace
						method="POST"
						action={withQuery("", { intent: "edit" })}
						onSubmit={() => adjustTimeModalClose()}
					>
						<Stack>
							<Title order={3}>Adjust times</Title>
							<DateTimePicker
								label="Start time"
								required
								name="startTime"
								defaultValue={new Date(loaderData.startTime)}
							/>
							<DateTimePicker
								label="End time"
								required
								name="endTime"
								defaultValue={new Date(loaderData.endTime)}
							/>
							<Button
								variant="outline"
								type="submit"
								name="id"
								value={loaderData.entityId}
							>
								Submit
							</Button>
						</Stack>
					</Form>
				</Modal>
			) : null}
			<Container size="xs">
				<Stack>
					<Group justify="space-between" wrap="nowrap">
						<Title>{loaderData.name}</Title>
						<Menu shadow="md" position="bottom-end">
							<Menu.Target>
								<ActionIcon variant="transparent" loading={isWorkoutLoading}>
									<IconDotsVertical />
								</ActionIcon>
							</Menu.Target>
							<Menu.Dropdown>
								<Menu.Item
									onClick={async () => {
										setIsWorkoutLoading(true);
										const workout = await duplicateOldWorkout(
											loaderData.information,
											loaderData.name,
											loaderData.repeatedWorkout?.id,
										);
										startWorkout(workout);
										setIsWorkoutLoading(false);
									}}
									leftSection={<IconRepeat size={14} />}
								>
									Repeat
								</Menu.Item>
								<Menu.Item
									onClick={adjustTimeModalOpen}
									leftSection={<IconClockEdit size={14} />}
								>
									Adjust time
								</Menu.Item>
								<Form
									method="POST"
									action={withQuery("", { intent: "delete" })}
								>
									<input
										type="hidden"
										name="workoutId"
										value={loaderData.entityId}
									/>
									<Menu.Item
										onClick={async (e) => {
											const form = e.currentTarget.form;
											e.preventDefault();
											const conf = await confirmWrapper({
												confirmation:
													"Are you sure you want to delete this workout? This action is not reversible.",
											});
											if (conf && form) submit(form);
										}}
										color="red"
										leftSection={<IconTrash size={14} />}
										type="submit"
									>
										Delete
									</Menu.Item>
								</Form>
							</Menu.Dropdown>
						</Menu>
					</Group>
					{loaderData.repeatedWorkout ? (
						<Box>
							<Text c="dimmed" span>
								Repeated from{" "}
							</Text>
							<Anchor
								component={Link}
								to={$path("/fitness/:entity/:id", {
									entity: "workout",
									id: loaderData.repeatedWorkout.id,
								})}
							>
								{loaderData.repeatedWorkout.name}
							</Anchor>
							<Text c="dimmed" span>
								{" "}
								on {dayjsLib(loaderData.repeatedWorkout.doneOn).format("LLL")}
							</Text>
						</Box>
					) : null}
					<Box>
						<Text c="dimmed" span>
							Done on{" "}
						</Text>
						<Text span>{dayjsLib(loaderData.startTime).format("LLL")}</Text>
						{loaderData.summary.total ? (
							<SimpleGrid mt="xs" cols={{ base: 3, md: 4, xl: 5 }}>
								{loaderData.endTime && loaderData.startTime ? (
									<DisplayStat
										icon={<IconClock size={16} />}
										data={humanizeDuration(
											new Date(loaderData.endTime).valueOf() -
												new Date(loaderData.startTime).valueOf(),
											{ round: true, units: ["h", "m"] },
										)}
									/>
								) : null}
								{Number(loaderData.summary.total.weight) !== 0 ? (
									<DisplayStat
										icon={<IconWeight size={16} />}
										data={displayWeightWithUnit(
											unitSystem,
											loaderData.summary.total.weight,
										)}
									/>
								) : null}
								{Number(loaderData.summary.total.distance) > 0 ? (
									<DisplayStat
										icon={<IconRun size={16} />}
										data={displayDistanceWithUnit(
											unitSystem,
											loaderData.summary.total.distance,
										)}
									/>
								) : null}
								<DisplayStat
									icon={<IconBarbell size={16} />}
									data={`${loaderData.summary.exercises.length} Exercises`}
								/>
								{Number(loaderData.summary.total.personalBestsAchieved) !==
								0 ? (
									<DisplayStat
										icon={<IconTrophy size={16} />}
										data={`${loaderData.summary.total.personalBestsAchieved} PRs`}
									/>
								) : null}
								{loaderData.summary.total.restTime > 0 ? (
									<DisplayStat
										icon={<IconZzz size={16} />}
										data={humanizeDuration(
											loaderData.summary.total.restTime * 1e3,
											{ round: true, units: ["m", "s"] },
										)}
									/>
								) : null}
							</SimpleGrid>
						) : null}
					</Box>
					{loaderData.information.comment ? (
						<Box>
							<Text c="dimmed" span>
								Commented:{" "}
							</Text>
							<Text span>{loaderData.information.comment}</Text>
						</Box>
					) : null}
					{loaderData.information.exercises.length > 0 ? (
						loaderData.information.exercises.map((exercise, idx) => (
							<DisplayExercise
								key={`${exercise.name}-${idx}`}
								exercise={exercise}
								idx={idx}
							/>
						))
					) : (
						<Paper withBorder p="xs">
							No exercises done
						</Paper>
					)}
				</Stack>
			</Container>
		</>
	);
}

type Exercise =
	WorkoutDetailsQuery["workoutDetails"]["information"]["exercises"][number];

const DisplayExercise = (props: { exercise: Exercise; idx: number }) => {
	const loaderData = useLoaderData<typeof loader>();
	const unitSystem = useUserUnitSystem();
	const [opened, { toggle }] = useDisclosure(false);
	const [parent] = useAutoAnimate();
	const { data: exerciseDetails } = useQuery(
		getExerciseDetailsQuery(props.exercise.name),
	);

	const supersetLinks =
		props.exercise.supersetWith.length > 0
			? props.exercise.supersetWith
					.map<ReactNode>((otherExerciseIdx) => (
						<Anchor
							key={otherExerciseIdx}
							fz="xs"
							href={withFragment(
								"",
								`${loaderData.information.exercises[otherExerciseIdx].name}__${otherExerciseIdx}`,
							)}
						>
							{loaderData.information.exercises[otherExerciseIdx].name}
						</Anchor>
					))
					.reduce((prev, curr) => [prev, ", ", curr])
			: null;

	return (
		<Paper withBorder p="xs">
			<Stack mb="xs" gap="xs" ref={parent}>
				<Group justify="space-between" wrap="nowrap">
					<Anchor
						id={props.idx.toString()}
						component={Link}
						to={$path("/fitness/exercises/item/:id", {
							id: props.exercise.name,
						})}
						fw="bold"
						lineClamp={1}
						style={{ scrollMargin: 20 }}
					>
						{props.exercise.name}
					</Anchor>
					<ActionIcon onClick={toggle} variant="transparent">
						<IconInfoCircle size={18} />
					</ActionIcon>
				</Group>
				{opened ? (
					<>
						<SimpleGrid cols={{ base: 2, md: 3 }} spacing={4}>
							{props.exercise.restTime ? (
								<Flex align="center" gap="xs">
									<IconZzz size={14} />
									<Text fz="xs">Rest time: {props.exercise.restTime}s</Text>
								</Flex>
							) : null}
							{props.exercise.total ? (
								<>
									{Number(props.exercise.total.reps) > 0 ? (
										<Flex align="center" gap="xs">
											<IconRotateClockwise size={14} />
											<Text fz="xs">Reps: {props.exercise.total.reps}</Text>
										</Flex>
									) : null}
									{Number(props.exercise.total.duration) > 0 ? (
										<Flex align="center" gap="xs">
											<IconClock size={14} />
											<Text fz="xs">
												Duration: {props.exercise.total.duration} min
											</Text>
										</Flex>
									) : null}
									{Number(props.exercise.total.weight) > 0 ? (
										<Flex align="center" gap="xs">
											<IconWeight size={14} />
											<Text fz="xs">
												Weight:{" "}
												{displayWeightWithUnit(
													unitSystem,
													props.exercise.total.weight,
												)}
											</Text>
										</Flex>
									) : null}
									{Number(props.exercise.total.distance) > 0 ? (
										<Flex align="center" gap="xs">
											<IconRun size={14} />
											<Text fz="xs">
												Distance:{" "}
												{displayDistanceWithUnit(
													unitSystem,
													props.exercise.total.distance,
												)}
											</Text>
										</Flex>
									) : null}
								</>
							) : null}
						</SimpleGrid>
						{exerciseDetails ? (
							<ScrollArea type="scroll">
								<Flex gap="lg">
									{exerciseDetails.attributes.images.map((i) => (
										<Image key={i} radius="md" src={i} h={200} w={350} />
									))}
								</Flex>
							</ScrollArea>
						) : null}
					</>
				) : null}
				{supersetLinks ? (
					<Text fz="xs">Superset with {supersetLinks}</Text>
				) : null}
				{props.exercise.notes.map((n, idxN) => (
					<Text c="dimmed" key={n} size="xs">
						{props.exercise.notes.length === 1 ? undefined : `${idxN + 1})`} {n}
					</Text>
				))}
				{props.exercise.assets && props.exercise.assets.images.length > 0 ? (
					<Avatar.Group>
						{props.exercise.assets.images.map((i) => (
							<Anchor key={i} href={i} target="_blank">
								<Avatar src={i} />
							</Anchor>
						))}
					</Avatar.Group>
				) : null}
			</Stack>
			{props.exercise.sets.map((set, idx) => (
				<DisplaySet
					set={set}
					idx={idx}
					key={set.confirmedAt}
					exerciseLot={props.exercise.lot}
				/>
			))}
		</Paper>
	);
};

const DisplayStat = (props: { icon: ReactNode; data: string }) => {
	return (
		<Stack gap={4} align="center" justify="center">
			{props.icon}
			<Text span size="sm" ta="center">
				{props.data}
			</Text>
		</Stack>
	);
};
