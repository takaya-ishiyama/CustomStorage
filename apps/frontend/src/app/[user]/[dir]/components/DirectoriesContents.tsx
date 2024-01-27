"use client";
import { userAtom } from "@/app/hooks/jotai/user/atom";
import { getRoutes } from "@/app/routes";
import { useGetOwnDirectories } from "@/infrastructure/Query/service/useGetOwnDirectories";
import { Box, Button } from "@radix-ui/themes";
import { useAtomValue } from "jotai";
import { usePathname, useRouter } from "next/navigation";
import React from "react";
import { useCreateDirectory } from "../../home/hooks/useCreateDirectory";
import { FullWithModal } from "@/app/components/molecules";

export const DirectoriesContents = () => {
	const user = useAtomValue(userAtom);
	const router = useRouter();
	const pathnameList = usePathname().split("/");

	const { data, isLoading, refetch } = useGetOwnDirectories({
		userId: user.id ?? "",
		pearentId: pathnameList[2],
		options: { retry: 3 },
	});

	const { createDirectory, isLoading: isMutate } = useCreateDirectory({
		refetchDirectories: refetch,
		parentId: pathnameList[2],
	});

	const navigateToDirectory = React.useCallback(
		(id: string) =>
			router.push(getRoutes.owndir({ userId: user.id ?? "", dirId: id })),
		[router, user.id],
	);

	return (
		<>
			{/* <CreateDirectory /> */}
			<FullWithModal isOpen={false} />

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
