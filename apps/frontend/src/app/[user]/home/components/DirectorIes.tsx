"use client";

import { userAtom } from "@/app/hooks/jotai/user/atom";
import { Box, Button } from "@radix-ui/themes";
import { useAtomValue } from "jotai";
import React from "react";
import { useCreateDirectory } from "../hooks/useCreateDirectory";
import { useGetRootDirectory } from "@/infrastructure/Query/service/useGetRootDirectory";
import { Loading } from "@/app/components/molecules/Loading";

export const DirectoriesList = () => {
	const user = useAtomValue(userAtom);
	const { data, isLoading, refetch } = useGetRootDirectory({
		userId: user.id ?? "",
		options: { retry: 3 },
	});
	const { createDirectory, isLoading: isMutate } = useCreateDirectory({
		refetchDirectories: refetch,
		parentId: null,
	});

	if (isLoading) return <Loading />;
	return (
		<>
			<Button
				disabled={isMutate}
				onClick={() => {
					createDirectory({ name: "test" });
				}}
			>
				CreateDir後で場所を移す
			</Button>

			<Box>
				{data?.directories?.map((d) => (
					<div key={d.id}>{d.name}</div>
				))}
			</Box>
		</>
	);
};
