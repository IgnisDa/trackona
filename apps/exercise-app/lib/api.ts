import AsyncStorage from "@react-native-async-storage/async-storage";
import { GraphQLClient, createGqlClient } from "@ryot/graphql/src/client";
import { QueryClient } from "@tanstack/react-query";

let gqlClient: GraphQLClient;
const KEY = "instanceUrl";

export const getGraphqlClient = async () => {
	if (gqlClient) return gqlClient;
	if (typeof AsyncStorage !== "undefined") {
		const baseUrl = await AsyncStorage.getItem(KEY);
		if (!baseUrl) return;
		gqlClient = createGqlClient(baseUrl);
		return gqlClient;
	}
	return;
};

export const queryClient = new QueryClient();
