export const APP_ROUTES = {
	dashboard: "/",
	calendar: "/calendar",
	auth: {
		login: "/auth/login",
		register: "/auth/register",
	},
	settings: {
		profile: "/settings/profile",
		preferences: "/settings/preferences",
		integrations: "/settings/integrations",
		notifications: "/settings/notifications",
		miscellaneous: "/settings/miscellaneous",
		users: "/settings/users",
		imports: {
			new: "/settings/imports-and-exports",
			reports: "/settings/imports-and-exports/reports",
		},
	},
	media: {
		list: "/media/list",
		postReview: "/media/post-review",
		collections: {
			list: "/media/collections/list",
			details: "/media/collections",
		},
		groups: {
			list: "/media/groups/list",
			details: "/media/groups",
		},
		people: {
			list: "/media/people/list",
			details: "/media/people",
		},
		individualMediaItem: {
			details: "/media/item",
			commit: "/media/item/commit",
			create: "/media/item/create",
			updateProgress: "/media/item/update-progress",
		},
	},
	fitness: {
		exercises: {
			currentWorkout: "/fitness/exercises/current-workout",
			list: "/fitness/exercises/list",
			details: "/fitness/exercises/details",
		},
		measurements: "/fitness/measurements",
		workouts: "/fitness/exercises/workouts/list",
	},
} as const;

export const LOCAL_STORAGE_KEYS = {
	colorScheme: "mantine-color-scheme",
	currentWorkout: "currentWorkout",
	savedCalendarDay: "1",
	savedMeasurementsDisplaySelectedStats: "2",
	savedMeasurementsDisplaySelectedTimespan: "3",
	savedActiveExerciseDetailsTab: "4",
	savedExercisesPage: "5",
	savedExercisesQuery: "6",
	savedExerciseFilters: "7",
	savedExerciseSortBy: "8",
	savedWorkoutListPage: "9",
	savedMineMediaSortOrder: "10",
	savedMineMediaSortBy: "11",
	savedMineMediaGeneralFilter: "12",
	savedMineMediaCollectionFilter: "13",
	savedMediaSearchPage: "14",
	savedMediaQuery: "15",
	savedMediaSearchSource: "16",
	savedMediaMinePage: "17",
	savedMediaActiveTab: "18",
	savedCollectionPage: "19",
	savedGroupsQuery: "20",
	savedGroupsPage: "21",
	savedActiveItemDetailsTab: "22",
	savedActiveCreatorDetailsTab: "23",
	savedCreatorsQuery: "24",
	savedCreatorPage: "25",
	savedCreatorSortBy: "26",
	savedCreatorSortOrder: "27",
	savedPreferencesTab: "28",
} as const;
