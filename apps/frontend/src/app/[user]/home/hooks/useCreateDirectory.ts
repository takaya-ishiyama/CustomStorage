import { userAtom } from "@/app/hooks/jotai/user/atom";
import { useCreateDirectoryMutation } from "@/infrastructure/Query/service/useCreateDirectory";
import { ServiceSchema } from "@/infrastructure/graphql/graphql";
import { AxiosError } from "axios";
import { useAtomValue } from "jotai";
import React from "react";
import { QueryObserverResult } from "react-query";

type Props = {
	refetchDirectories: () => Promise<
		QueryObserverResult<ServiceSchema, AxiosError<unknown>>
	>;
	parentId: string | null;
};

type CreateDirectoryProps = {
	name: string;
};

export const useCreateDirectory = ({ refetchDirectories, parentId }: Props) => {
	const user = useAtomValue(userAtom);
	const { data, mutate, isLoading } = useCreateDirectoryMutation({
		options: {
			onSuccess: async () => {
				await refetchDirectories();
			},
		},
	});
	const createDirectory = React.useCallback(
		(input: CreateDirectoryProps) => {
			mutate({
				name: input.name,
				parentId: parentId,
				userId: user.id ?? "",
			});
		},
		[mutate, parentId, user.id],
	);

	return {
		handleClick,
		isLoading,
	};
};
