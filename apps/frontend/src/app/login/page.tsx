"use client";

import React from "react";
import { ProviderWrapper } from "../hooks";
import { LoginFormContents } from "./components/LoginFormContents";
import { CustomHeader } from "../components/features/CustomHeader";

const page = () => {
	return (
		<ProviderWrapper>
			<CustomHeader />
			<LoginFormContents />
		</ProviderWrapper>
	);
};

export default page;
