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
	const { handleClick, isLoading: isMutate } = useCreateDirectory();
	const { data, isLoading } = useGetRootDirectory({
		userId: user.id ?? "",
		options: { retry: 3 },
	});

	if (isLoading) return <Loading />;
	return (
		<>
			<Button
				disabled={isMutate}
				onClick={() => {
					handleClick({ userId: user.id ?? "", name: "test", parentId: null });
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
