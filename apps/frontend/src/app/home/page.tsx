"use client";

import React from "react";
import { MainContents } from "./components/MainContents";
import { QueryClient, QueryClientProvider } from "react-query";
import { CustomHeader } from "../components/features/CustomHeader";
import { ProviderWrapper } from "../hooks/ProviderWrapper";

const page = () => {
	return (
		<ProviderWrapper>
			<CustomHeader />
			<MainContents />
		</ProviderWrapper>
	);
};

export default page;
