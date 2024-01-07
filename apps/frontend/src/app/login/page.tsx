"use client";

import React from "react";
import { ProviderWrapper } from "../hooks/Provider/ProviderWrapper";
import { LoginFormContents } from "./components/LoginFormContents";
import { useAtomValue } from "jotai";
import { userAtom } from "../hooks/jotai/user/atom";
import { useRouter } from "next/navigation";
import { Routes, useRoutes } from "../routes";
import { Loading } from "../components/molecules/Loading";
import { Flex } from "@radix-ui/themes";

const Login = () => {
	const user = useAtomValue(userAtom);
	const router = useRouter();
	const { getHome } = useRoutes();
	// React.useEffect(() => {
	// 	if (user.id != null && user.id !== "") router.push(getHome(user.id));
	// }, [getHome, router, user.id]);
	if (user === null || user === undefined) return <Loading />;
	return (
		<ProviderWrapper>
			<Flex>ログイン</Flex>
			<LoginFormContents />
		</ProviderWrapper>
	);
};

export default Login;
