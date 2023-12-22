"use client";

import React from "react";
import { MainContents } from "./components/MainContents";
import { QueryClient, QueryClientProvider } from "react-query";
import { ProviderWrapper } from "../hooks";

const queryClient = new QueryClient();

const page = () => {
	return (
		<ProviderWrapper>
			<MainContents />
		</ProviderWrapper>
	);
};

export default page;
