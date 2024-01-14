"use client";
import React from "react";
import { Box, Button } from "@radix-ui/themes";
import { useLoginWithToken } from "./hooks/auth/useLoginWithToken";
import { useRoutes } from "./routes";
import { useRouter } from "next/navigation";

export const LoginContents = () => {
	const router = useRouter();
	const { getLogin, getHome } = useRoutes();
	const { data, loginFn } = useLoginWithToken({
		onSuccess: () => {
			if (data?.id === undefined) {
				router.push(getLogin());
				return;
			}
			router.push(getHome(data.id));
		},
		onError: () => router.push(getLogin()),
	});
	const handleClickLogin = React.useCallback(async () => {
		await loginFn();
	}, [loginFn]);
	return (
		<Box>
			<Button onClick={handleClickLogin}>ログイン</Button>
		</Box>
	);
};
