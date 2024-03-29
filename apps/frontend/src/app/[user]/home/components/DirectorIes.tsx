"use client";

import { userAtom } from "@/app/hooks/jotai/user/atom";
import { Box, Button } from "@radix-ui/themes";
import { useAtomValue } from "jotai";
import React from "react";
import { useCreateDirectory } from "../hooks/useCreateDirectory";
import { Loading } from "@/app/components/molecules/Loading";
import { useGetOwnDirectories } from "@/infrastructure/Query/service/useGetOwnDirectories";
import { useRouter } from "next/navigation";
import { getRoutes } from "@/app/routes";

export const DirectoriesList = () => {
	const user = useAtomValue(userAtom);
	const router = useRouter();

	const navigateToDirectory = React.useCallback(
		(id: string) =>
			router.push(getRoutes.owndir({ userId: user.id ?? "", dirId: id })),
		[router, user.id],
	);
	const { data, isLoading, refetch } = useGetOwnDirectories({
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
					// バリデーションするラッパーに囲んでhandleClick~という命名にする
					createDirectory({ name: "test" });
				}}
			>
				CreateDir後で場所を移す
			</Button>

			<Box>
				{data?.directories?.map((d) => (
					<Box key={d.id}>
						<Button
							onClick={() => {
								navigateToDirectory(d.id);
							}}
						>
							{d.name}
						</Button>
					</Box>
				))}
			</Box>
		</>
	);
};
