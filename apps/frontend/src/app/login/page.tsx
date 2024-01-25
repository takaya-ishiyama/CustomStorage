"use client";

import React from "react";
import { ProviderWrapper } from "../hooks/Provider/ProviderWrapper";
import { LoginFormContents } from "./components/LoginFormContents";
import { useAtomValue } from "jotai";
import { userAtom } from "../hooks/jotai/user/atom";
import { Loading } from "../components/molecules/Loading";
import { Flex } from "@radix-ui/themes";
import { AuthCheckProvider } from "../hooks/Provider/AuthCheckProvider";

const Login = () => {
	const user = useAtomValue(userAtom);
	if (user === null || user === undefined) return <Loading />;
	return (
		<ProviderWrapper>
			{/* <AuthCheckProvider> */}
			<Flex>ログイン</Flex>
			<LoginFormContents />
			{/* </AuthCheckProvider> */}
		</ProviderWrapper>
	);
};

export default Login;
