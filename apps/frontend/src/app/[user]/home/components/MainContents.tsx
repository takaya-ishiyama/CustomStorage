import { userAtom } from "@/app/hooks/jotai/user/atom";
import { useAtomValue } from "jotai";
import React from "react";
import { DirectoriesList } from "./DirectorIes";
import { Box } from "@radix-ui/themes";

export const MainContents: React.FC = () => {
	return (
		<>
			<Box>aaaaaaaaaaaa</Box>
			<DirectoriesList />
		</>
	);
};
