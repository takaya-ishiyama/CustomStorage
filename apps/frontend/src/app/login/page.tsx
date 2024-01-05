"use client";

import React from "react";
import { CustomHeader } from "../components/features/CustomHeader";
import { ProviderWrapper } from "../hooks/Provider/ProviderWrapper";
import { LoginFormContents } from "./components/LoginFormContents";
import { Box } from "@mui/material";
import { useAtomValue } from "jotai";
import { userAtom } from "../hooks/jotai/user/atom";
import { useRouter } from "next/navigation";
import { Routes } from "../routes";
import { Loading } from "../components/molecules/Loading";

const Login = () => {
	const user = useAtomValue(userAtom);
	const router = useRouter();
	React.useEffect(() => {
		console.log("eeecccd", user);
		if (user.id != null && user.id !== "") router.push(Routes.home);
	}, [user, router]);
	if (user === null || user === undefined) return <Loading />;
	return (
		<ProviderWrapper>
			<CustomHeader />
			<Box>ログイン</Box>
			<LoginFormContents />
		</ProviderWrapper>
	);
};

export default Login;
