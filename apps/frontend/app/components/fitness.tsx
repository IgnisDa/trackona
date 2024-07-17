import { $path } from "@ignisda/remix-routes";
import { Anchor, Flex, Paper, Text } from "@mantine/core";
import { Link } from "@remix-run/react";
import {
	ExerciseLot,
	SetLot,
	type UserExerciseDetailsQuery,
	UserUnitSystem,
	type WorkoutSetStatistic,
} from "@ryot/generated/graphql/backend/graphql";
import { truncate } from "@ryot/ts-utils";
import { useQuery } from "@tanstack/react-query";
import { match } from "ts-pattern";
import { withFragment } from "ufo";
import { dayjsLib, getSetColor } from "~/lib/generals";
import { useUserUnitSystem } from "~/lib/hooks";
import { getWorkoutDetailsQuery } from "~/lib/state/fitness";

export const getSetStatisticsTextToDisplay = (
	lot: ExerciseLot,
	statistic: WorkoutSetStatistic,
	unit: UserUnitSystem,
) => {
	return match(lot)
		.with(ExerciseLot.DistanceAndDuration, () => [
			`${displayDistanceWithUnit(unit, statistic.distance)} for ${Number(
				statistic.duration,
			).toFixed(2)} min`,
			`${displayDistanceWithUnit(unit, statistic.pace)}/min`,
		])
		.with(ExerciseLot.Duration, () => [
			`${Number(statistic.duration).toFixed(2)} min`,
			undefined,
		])
		.with(ExerciseLot.Reps, () => [`${statistic.reps} reps`, undefined])
		.with(ExerciseLot.RepsAndWeight, () => [
			statistic.weight && statistic.weight !== "0"
				? `${displayWeightWithUnit(unit, statistic.weight)} × ${statistic.reps}`
				: `${statistic.reps} reps`,
			statistic.oneRm ? `${Number(statistic.oneRm).toFixed(1)} RM` : null,
		])
		.exhaustive();
};

/**
 * Display the correct weight unit for a given unit.
 */
export const displayWeightWithUnit = (
	unit: UserUnitSystem,
	data: string | number | null | undefined,
	compactNotation?: boolean,
) => {
	return new Intl.NumberFormat("en-us", {
		style: "unit",
		unit: unit === UserUnitSystem.Metric ? "kilogram" : "pound",
		notation: compactNotation ? "compact" : undefined,
	}).format(Number((data || 0).toString()));
};

/**
 * Display the correct distance unit for a given unit.
 */
export const displayDistanceWithUnit = (
	unit: UserUnitSystem,
	data: string | number | null | undefined,
) => {
	return new Intl.NumberFormat("en-us", {
		style: "unit",
		unit: unit === UserUnitSystem.Metric ? "kilometer" : "mile",
	}).format(Number((data || 0).toString()));
};

/**
 * Display statistics for a set.
 **/
export const DisplaySetStatistics = (props: {
	lot: ExerciseLot;
	statistic: WorkoutSetStatistic;
	hideExtras?: boolean;
	centerText?: boolean;
}) => {
	const unitSystem = useUserUnitSystem();
	const [first, second] = getSetStatisticsTextToDisplay(
		props.lot,
		props.statistic,
		unitSystem,
	);

	return (
		<>
			<Text
				fz={props.hideExtras ? "xs" : "sm"}
				ta={props.centerText ? "center" : undefined}
			>
				{first}
			</Text>
			{!props.hideExtras && second ? (
				<Text
					ml="auto"
					fz={props.hideExtras ? "xs" : "sm"}
					ta={props.centerText ? "center" : undefined}
				>
					{second}
				</Text>
			) : null}
		</>
	);
};

export const ExerciseHistory = (props: {
	exerciseId: string;
	exerciseLot: ExerciseLot;
	history: NonNullable<
		UserExerciseDetailsQuery["userExerciseDetails"]["history"]
	>[number];
}) => {
	const { data } = useQuery(getWorkoutDetailsQuery(props.history.workoutId));

	return (
		<Paper key={props.history.workoutId} withBorder p="xs">
			<Anchor
				component={Link}
				to={withFragment(
					$path("/fitness/workouts/:id", { id: props.history.workoutId }),
					props.history.idx.toString(),
				)}
				fw="bold"
			>
				{truncate(data?.name, { length: 36 })}
			</Anchor>
			<Text c="dimmed" fz="sm" mb="xs">
				{dayjsLib(data?.endTime).format("LLLL")}
			</Text>
			{data?.information.exercises[props.history.idx].sets.map((s, idx) => (
				<Flex key={`${idx}-${s.lot}`} align="center">
					<Text fz="sm" c={getSetColor(s.lot)} mr="md" fw="bold" ff="monospace">
						{match(s.lot)
							.with(SetLot.Normal, () => idx + 1)
							.otherwise(() => s.lot.at(0))}
					</Text>
					<DisplaySetStatistics
						lot={props.exerciseLot}
						statistic={s.statistic}
					/>
				</Flex>
			))}
		</Paper>
	);
};
