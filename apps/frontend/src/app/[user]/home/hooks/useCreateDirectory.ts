import { userAtom } from "@/app/hooks/jotai/user/atom";
import { useCreateDirectoryMutation } from "@/infrastructure/Query/service/useCreateDirectory";
import { useAtomValue } from "jotai";
import React from "react";

type CreateDirectoryProps = {
	name: string;
	parentId: string | null;
	userId: string;
};

export const useCreateDirectory = () => {
	const user = useAtomValue(userAtom);
	const { data, mutate, isLoading } = useCreateDirectoryMutation({});
	const handleClick = React.useCallback(
		(input: CreateDirectoryProps) => {
			mutate({
				name: input.name,
				parentId: input.parentId,
				userId: input.userId,
			});
		},
		[mutate],
	);

	return {
		handleClick,
		isLoading,
	};
};
