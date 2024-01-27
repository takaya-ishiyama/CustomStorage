"use client";
import { AuthCheckProvider } from "@/app/hooks/Provider/AuthCheckProvider";
import React from "react";
import { DirectoriesContents } from "./components/DirectoriesContents";
import { ProviderWrapper } from "@/app/hooks/Provider/ProviderWrapper";
import { Test } from "./components/Test";

const page = () => {
	return (
		<>
			<ProviderWrapper>
				<AuthCheckProvider>
					<DirectoriesContents />
				</AuthCheckProvider>
			</ProviderWrapper>
		</>
	);
};

export default page;
