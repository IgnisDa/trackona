import { gqlClient } from "../services/api";
import {
	CommitMediaDocument,
	type CommitMediaMutationVariables,
	CoreEnabledFeaturesDocument,
	MetadataLot,
	UserDetailsDocument,
	UserEnabledFeaturesDocument,
} from "@ryot/generated/graphql/backend/graphql";
import { useMutation, useQuery } from "@tanstack/react-query";
import invariant from "tiny-invariant";

export function useUser() {
	const userDetails = useQuery({
		queryKey: ["userDetails"],
		queryFn: async () => {
			const { userDetails } = await gqlClient.request(UserDetailsDocument);
			return userDetails;
		},
	});
	return userDetails.data?.__typename === "User" ? userDetails.data : undefined;
}

export function useEnabledUserFeatures() {
	const enabledFeatures = useQuery(
		["enabledUserFeatures"],
		async () => {
			const { userEnabledFeatures } = await gqlClient.request(
				UserEnabledFeaturesDocument,
			);
			return userEnabledFeatures;
		},
		{ staleTime: Infinity },
	);
	return enabledFeatures;
}

export function useEnabledCoreFeatures() {
	const enabledFeatures = useQuery(
		["enabledCoreFeatures"],
		async () => {
			const { coreEnabledFeatures } = await gqlClient.request(
				CoreEnabledFeaturesDocument,
			);
			return coreEnabledFeatures;
		},
		{ staleTime: Infinity },
	);
	return enabledFeatures;
}

export function useCommitMedia(
	lot?: MetadataLot,
	onSuccess?: (id: any) => void,
) {
	const commitMedia = useMutation({
		mutationFn: async (variables: CommitMediaMutationVariables) => {
			invariant(lot, "Lot must be defined");
			const { commitMedia } = await gqlClient.request(
				CommitMediaDocument,
				variables,
			);
			return commitMedia;
		},
		onSuccess: (data) => {
			if (onSuccess) onSuccess(data.id);
		},
	});
	return commitMedia;
}
