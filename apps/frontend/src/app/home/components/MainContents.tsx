import { userAtom } from "@/app/hooks/jotai/atom";
import { useAtomValue } from "jotai";
import React from "react";

export const MainContents: React.FC = () => {
	const user = useAtomValue(userAtom);

	return <div>{user ? <div>{user.username}</div> : <></>}</div>;
};
