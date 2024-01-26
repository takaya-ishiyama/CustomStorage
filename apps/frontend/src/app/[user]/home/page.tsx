"use client";

import React from "react";
import { MainContents } from "./components/MainContents";
import { QueryClient, QueryClientProvider } from "react-query";
import { ProviderWrapper } from "../../hooks/Provider/ProviderWrapper";
import { AuthCheckProvider } from "@/app/hooks/Provider/AuthCheckProvider";

const page = () => {
	return (
		<ProviderWrapper>
			<AuthCheckProvider>
				<MainContents />
			</AuthCheckProvider>
		</ProviderWrapper>
	);
};

export default page;
