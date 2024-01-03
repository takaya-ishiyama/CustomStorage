"use client";

import React from "react";
import { MainContents } from "./components/MainContents";
import { QueryClient, QueryClientProvider } from "react-query";
import { ProviderWrapper } from "../hooks";
import { CustomHeader } from "../components/features/CustomHeader";

const page = () => {
	return (
		<ProviderWrapper>
			<CustomHeader />
			<MainContents />
		</ProviderWrapper>
	);
};

export default page;
