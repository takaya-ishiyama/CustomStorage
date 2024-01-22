import { AxiosError } from "axios";
import {
	UseMutationOptions,
	UseMutationResult,
	useMutation,
} from "react-query";
import { base_uri } from "../backendUri";
import {
	DirectorySchema,
	MutationCreateDirectoryArgs,
} from "@/infrastructure/graphql/graphql";
import { mutation } from "@/infrastructure/graphql/requestBody";

type CreateDirectoryProps = {
	options?: UseMutationOptions<
		DirectorySchema,
		AxiosError,
		MutationCreateDirectoryArgs,
		undefined
	>;
};

type CreateDirectoryQuery = ({
	options,
}: CreateDirectoryProps) => UseMutationResult<
	DirectorySchema,
	AxiosError,
	MutationCreateDirectoryArgs,
	undefined
>;

export const useCreateDirectoryMutation: CreateDirectoryQuery = ({
	options,
}) => {
	return useMutation(
		async (input: MutationCreateDirectoryArgs): Promise<DirectorySchema> => {
			const shcema = mutation.create_directory();
			const resp = await fetch(base_uri, {
				method: "POST",
				headers: {
					Accept: "application/json",
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					query: shcema,
					variables: { ...input },
				}),
			});
			if (!resp.ok) {
				throw new Error("Error");
			}
			const {
				data: { createDirectory },
			} = await resp.json();

			return createDirectory;
		},
		options,
	);
};
