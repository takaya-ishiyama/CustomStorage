"use client";

import React from "react";
import { CustomHeader } from "../components/features/CustomHeader";
import { ProviderWrapper } from "../hooks/Provider/ProviderWrapper";
import { LoginFormContents } from "./components/LoginFormContents";

const Login = () => {
	return (
		<ProviderWrapper>
			<CustomHeader />
			<LoginFormContents />
		</ProviderWrapper>
	);
};

export default Login;
