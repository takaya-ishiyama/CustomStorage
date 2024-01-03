import { useLogin } from "@/infrastructure/Query/authorization";
import { useUserQuery } from "@/infrastructure/Query/useUserQuery";
import { Button } from "@mui/material";
import React from "react";

export const MainContents = () => {
	const [user, setUser] = React.useState();
	// const { data, isLoading } = useUserQuery({});

	// React.useEffect(() => {
	// 	console.log("aaaa", data);
	// }, [data]);

	const { mutate, data: loginUser } = useLogin({});

	const handleClickLogin = () => {
		mutate({
			username: "test",
			password: "password",
		});
	};

	return (
		<div>
			<Button onClick={handleClickLogin}>{"aaaaa"}</Button>
			{loginUser ? <div>{loginUser?.username}</div> : <></>}
		</div>
	);
};
