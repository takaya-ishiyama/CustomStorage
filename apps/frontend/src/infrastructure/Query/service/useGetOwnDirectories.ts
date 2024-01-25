import {
	QueryGetOwnDirectoriesArgs,
	ServiceSchema,
} from "@/infrastructure/graphql/graphql";
import { query } from "@/infrastructure/graphql/requestBody";
import { AxiosError } from "axios";
import { UseQueryOptions, UseQueryResult, useQuery } from "react-query";
import { base_uri } from "../backendUri";

type Props = {
	options?: UseQueryOptions<ServiceSchema, AxiosError, ServiceSchema, string[]>;
} & QueryGetOwnDirectoriesArgs;

export const useGetOwnDirectories = ({
	userId,
	pearentId,
	options,
}: Props): UseQueryResult<ServiceSchema, AxiosError> => {
	return useQuery(
		["rootDirectories"],
		async (context): Promise<ServiceSchema> => {
			const shcema = query.get_own_directories();
			const res = await fetch(base_uri, {
				method: "POST",
				headers: {
					Accept: "application/json",
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					query: shcema,
					variables: { userId, pearentId },
				}),
			});

			if (!res.ok) {
				throw new Error(`HTTP error! Status: ${res.status}`);
			}
			const { data } = await res.json();
			return data.getOwnDirectories;
		},
		options,
	);
};
