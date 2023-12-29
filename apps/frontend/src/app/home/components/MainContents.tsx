import { useUserQuery } from "@/infrastructure/Query/useUserQuery";
import { Button } from "@mui/material";
import React from "react";

export const MainContents = () => {
	const { data, isLoading } = useUserQuery({});

	React.useEffect(() => {
		console.log("aaaa", data);
	}, [data]);

	if (isLoading) {
		return <div>loading...</div>;
	}

	return (
		<div>
			<Button onClick={() => {}}>{data?.username}</Button>
		</div>
	);
};
