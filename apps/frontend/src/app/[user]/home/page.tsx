"use client";

import React from "react";
import { MainContents } from "./components/MainContents";
import { QueryClient, QueryClientProvider } from "react-query";
import { ProviderWrapper } from "../../hooks/Provider/ProviderWrapper";

const page = () => {
	return (
		<ProviderWrapper>
			{/* <AuthProvider> */}
			{/* <CustomHeader /> */}
			<MainContents />
			{/* </AuthProvider> */}
		</ProviderWrapper>
	);
};

export default page;
