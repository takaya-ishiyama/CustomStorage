"use client";

import { userAtom } from "@/app/hooks/jotai/user/atom";
import { Button } from "@radix-ui/themes";
import { useAtomValue } from "jotai";
import React from "react";
import { useCreateDirectory } from "../hooks/useCreateDirectory";

export const DirectoriesList = () => {
	const user = useAtomValue(userAtom);
	const { handleClick, isLoading } = useCreateDirectory();

	return (
		<>
			<Button onClick={() => {}}>Button</Button>
		</>
	);
};
