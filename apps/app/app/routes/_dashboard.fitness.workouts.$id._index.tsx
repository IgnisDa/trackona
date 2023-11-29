import {
	ActionIcon,
	Anchor,
	Avatar,
	Badge,
	Box,
	Button,
	Container,
	Flex,
	Group,
	Menu,
	Modal,
	Paper,
	Stack,
	Text,
	Title,
} from "@mantine/core";
import { DateTimePicker } from "@mantine/dates";
import { useDisclosure } from "@mantine/hooks";
import { LoaderFunctionArgs, MetaFunction, json } from "@remix-run/node";
import { Form, Link, useLoaderData, useNavigate } from "@remix-run/react";
import {
	SetLot,
	WorkoutDetailsDocument,
} from "@ryot/generated/graphql/backend/graphql";
import { startCase } from "@ryot/ts-utils";
import {
	IconClock,
	IconClockEdit,
	IconDotsVertical,
	IconRepeat,
	IconTrash,
	IconTrophy,
	IconWeight,
	IconZzz,
} from "@tabler/icons-react";
import {
	HumanizeDuration,
	HumanizeDurationLanguage,
} from "humanize-duration-ts";
import { useAtom } from "jotai";
import { DateTime } from "luxon";
import { ReactElement } from "react";
import { $path } from "remix-routes";
import invariant from "tiny-invariant";
import { match } from "ts-pattern";
import { DisplayExerciseStats } from "~/components/fitness";
import { getAuthorizationHeader, gqlClient } from "~/lib/api.server";
import { useGetMantineColor } from "~/lib/hooks";
import { getSetColor } from "~/lib/utilities";
import { currentWorkoutAtom, duplicateOldWorkout } from "~/lib/workout";

const service = new HumanizeDurationLanguage();
const humanizer = new HumanizeDuration(service);

export const loader = async ({ request, params }: LoaderFunctionArgs) => {
	const workoutId = params.id;
	invariant(workoutId, "No ID provided");
	const [{ workoutDetails }] = await Promise.all([
		gqlClient.request(
			WorkoutDetailsDocument,
			{ workoutId },
			await getAuthorizationHeader(request),
		),
	]);
	return json({
		workoutId,
		workoutDetails,
	});
};

export const meta: MetaFunction = ({ data }) => {
	return [
		{
			title: `${
				// biome-ignore lint/suspicious/noExplicitAny:
				(data as any).workoutDetails.name
			} | Ryot`,
		},
	];
};

export default function Page() {
	const loaderData = useLoaderData<typeof loader>();
	const [_, setCurrentWorkout] = useAtom(currentWorkoutAtom);
	const getMantineColor = useGetMantineColor();
	const [
		adjustTimeModalOpened,
		{ open: adjustTimeModalOpen, close: adjustTimeModalClose },
	] = useDisclosure(false);
	const navigate = useNavigate();

	return (
		<>
			<Modal
				opened={adjustTimeModalOpened}
				onClose={adjustTimeModalClose}
				withCloseButton={false}
				centered
			>
				<Box component={Form} action="?intent=adjustTime" method="post">
					<Stack>
						<Title order={3}>Adjust times</Title>
						<DateTimePicker
							label="Start time"
							required
							name="startTime"
							defaultValue={new Date(loaderData.workoutDetails.startTime)}
						/>
						<DateTimePicker
							label="End time"
							required
							name="endTime"
							defaultValue={new Date(loaderData.workoutDetails.endTime)}
						/>
						<Button variant="outline" type="submit">
							Adjust
						</Button>
					</Stack>
				</Box>
			</Modal>
			<Container size="xs">
				<Stack>
					<Group justify="space-between">
						<Title>{loaderData.workoutDetails.name}</Title>
						<Menu shadow="md" position="bottom-end">
							<Menu.Target>
								<ActionIcon>
									<IconDotsVertical />
								</ActionIcon>
							</Menu.Target>
							<Menu.Dropdown>
								<Menu.Item
									onClick={() => {
										setCurrentWorkout(
											duplicateOldWorkout(loaderData.workoutDetails),
										);
										navigate($path("/fitness/workouts/current"));
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
								<Menu.Item
									onClick={() => {
										const yes = confirm(
											"Are you sure you want to delete this workout? This action is not reversible.",
										);
										if (yes) deleteWorkout.mutate({ workoutId });
									}}
									color="red"
									leftSection={<IconTrash size={14} />}
								>
									Delete
								</Menu.Item>
							</Menu.Dropdown>
						</Menu>
					</Group>
					<Box>
						<Text c="dimmed" span>
							Done on{" "}
						</Text>
						<Text span>
							{DateTime.fromISO(
								loaderData.workoutDetails.startTime,
							).toLocaleString(DateTime.DATETIME_MED)}
						</Text>
						<Group mt="xs" gap="lg">
							<DisplayStat
								icon={<IconClock size={16} />}
								data={humanizer.humanize(
									new Date(loaderData.workoutDetails.endTime).getTime() -
										new Date(loaderData.workoutDetails.startTime).getTime(),
									{ round: true, units: ["h", "m"] },
								)}
							/>
							<DisplayStat
								icon={<IconWeight size={16} />}
								data={new Intl.NumberFormat("en-us", {
									style: "unit",
									unit: "kilogram",
								}).format(
									Number(loaderData.workoutDetails.summary.total.weight),
								)}
							/>
							<DisplayStat
								icon={<IconTrophy size={16} />}
								data={`${loaderData.workoutDetails.summary.total.personalBestsAchieved.toString()} PRs`}
							/>
							{loaderData.workoutDetails.summary.total.restTime > 0 ? (
								<DisplayStat
									icon={<IconZzz size={16} />}
									data={humanizer.humanize(
										loaderData.workoutDetails.summary.total.restTime * 1e3,
										{ round: true, units: ["m", "s"] },
									)}
								/>
							) : undefined}
						</Group>
					</Box>
					{loaderData.workoutDetails.comment ? (
						<Box>
							<Text c="dimmed" span>
								Commented:{" "}
							</Text>
							<Text span>{loaderData.workoutDetails.comment}</Text>
						</Box>
					) : undefined}
					{loaderData.workoutDetails.information.exercises.length > 0 ? (
						loaderData.workoutDetails.information.exercises.map(
							(exercise, idx) => (
								<Paper key={`${exercise.id}-${idx}`} withBorder p="xs">
									<Box mb="xs">
										<Group justify="space-between">
											<Anchor
												component={Link}
												to={$path("/fitness/workouts/:id", { id: exercise.id })}
												fw="bold"
											>
												{exercise.id}
											</Anchor>
											{exercise.restTime ? (
												<Flex align="center" gap="xs">
													<IconZzz size={14} />
													<Text fz="xs">{exercise.restTime}s</Text>
												</Flex>
											) : undefined}
										</Group>
										{exercise.notes.map((n, idxN) => (
											<Text c="dimmed" key={n} size="xs">
												{exercise.notes.length === 1
													? undefined
													: `${idxN + 1})`}{" "}
												{n}
											</Text>
										))}
										{exercise.assets.images.length > 0 ? (
											<Avatar.Group>
												{exercise.assets.images.map((i) => (
													<Anchor
														key={i}
														href={i}
														target="_blank"
														rel="noopener noreferrer"
													>
														<Avatar src={i} />
													</Anchor>
												))}
											</Avatar.Group>
										) : undefined}
									</Box>
									{exercise.sets.map((s, idx) => (
										<Box key={`${idx}`} mb={2}>
											<Flex align="center">
												<Text
													fz="sm"
													c={getSetColor(s.lot)}
													mr="md"
													fw="bold"
													ff="monospace"
												>
													{match(s.lot)
														.with(SetLot.Normal, () => idx + 1)
														.otherwise(() => s.lot.at(0))}
												</Text>
												<DisplayExerciseStats
													lot={exercise.lot}
													statistic={s.statistic}
												/>
											</Flex>
											{s.personalBests.length > 0 ? (
												<Flex mb={6} mt={2} ml="lg">
													{s.personalBests.map((pb) => (
														<Badge
															variant="light"
															size="xs"
															leftSection={<IconTrophy size={16} />}
															color={getMantineColor(pb)}
														>
															{startCase(pb)}
														</Badge>
													))}
												</Flex>
											) : undefined}
										</Box>
									))}
								</Paper>
							),
						)
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

const DisplayStat = (props: {
	icon: ReactElement;
	data: string;
}) => {
	return (
		<Flex gap={4} align="center">
			{props.icon}
			<Text span size="sm">
				{props.data}
			</Text>
		</Flex>
	);
};
