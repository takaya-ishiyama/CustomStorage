import { userAtom } from "@/app/hooks/jotai/user/atom";
import { useAtomValue } from "jotai";
import React from "react";

export const MainContents: React.FC = () => {
	const user = useAtomValue(userAtom);
	React.useEffect(() => {
		console.log(user);
	}, [user]);

	return (
		<div>
			{user ? <div>{user.username}</div> : <div>ログインしてください</div>}
		</div>
	);
};
