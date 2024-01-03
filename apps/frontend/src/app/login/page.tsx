"use client";

import React from "react";
import { MainContents } from "./components/MainContents";
import { QueryClient, QueryClientProvider } from "react-query";
import { ProviderWrapper } from "../hooks";
import { Header } from "../components/organisms/Header";

const page = () => {
	return (
		<ProviderWrapper>
			<Header />
			<MainContents />
		</ProviderWrapper>
	);
};

export default page;
