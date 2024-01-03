"use client";

import React from "react";
import { CustomHeader } from "../components/features/CustomHeader";
import { ProviderWrapper } from "../hooks/ProviderWrapper";
import { LoginFormContents } from "./components/LoginFormContents";

const page = () => {
	return (
		<ProviderWrapper>
			<CustomHeader />
			<LoginFormContents />
		</ProviderWrapper>
	);
};

export default page;
