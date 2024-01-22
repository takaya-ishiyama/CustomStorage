import { ServiceSchema } from "@/infrastructure/graphql/graphql";
import { query } from "@/infrastructure/graphql/requestBody";
import { AxiosError } from "axios";
import { UseQueryOptions, UseQueryResult, useQuery } from "react-query";
import { base_uri } from "../backendUri";

export const useGetRootDirectory = ({
	userId,
	options,
}: {
	userId: string;
	options?: Omit<UseQueryOptions<ServiceSchema, AxiosError>, "queryKey">;
}): UseQueryResult<ServiceSchema, AxiosError> => {
	return useQuery(
		["rootDirectories"],
		async (): Promise<ServiceSchema> => {
			const shcema = query.get_root_directory();
			const res = await fetch(base_uri, {
				method: "POST",
				headers: {
					Accept: "application/json",
					"Content-Type": "application/json",
				},
				body: JSON.stringify({ query: shcema, variables: { userId } }),
			});

			if (!res.ok) {
				throw new Error(`HTTP error! Status: ${res.status}`);
			}
			const { data } = await res.json();
			return data.getRootDirectory;
		},
		options,
	);
};
