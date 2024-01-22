"use client";

import { userAtom } from "@/app/hooks/jotai/user/atom";
import { useCreateDirectoryMutation } from "@/infrastructure/Query/service/useCreateDirectory";
import { Button } from "@radix-ui/themes";
import { useAtomValue } from "jotai";
import React from "react";

export const DirectoriesList = () => {
	const user = useAtomValue(userAtom);
	const { data, mutate, isLoading } = useCreateDirectoryMutation({});
	const handleClick = React.useCallback(() => {
		mutate({
			name: "test",
			parentId: null,
			userId: user.id ?? "",
		});
	}, [mutate, user.id]);
	return (
		<>
			<Button onClick={handleClick}>Button</Button>
			{data && <div>{data.name}</div>}
		</>
	);
};
