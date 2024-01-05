"use client";

import React from "react";
import { MainContents } from "./components/MainContents";
import { QueryClient, QueryClientProvider } from "react-query";
import { CustomHeader } from "../components/features/CustomHeader";
import { ProviderWrapper } from "../hooks/Provider/ProviderWrapper";
import { AuthProvider } from "../hooks/Provider/AuthProvider";

const page = () => {
	return (
		<ProviderWrapper>
			<AuthProvider>
				<CustomHeader />
				<MainContents />
			</AuthProvider>
		</ProviderWrapper>
	);
};

export default page;
