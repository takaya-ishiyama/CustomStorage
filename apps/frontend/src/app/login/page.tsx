"use client";

import React from "react";
import { ProviderWrapper } from "../hooks";
import { Header } from "../components/organisms/Header";
import { LoginFormContents } from "./components/LoginFormContents";

const page = () => {
	return (
		<ProviderWrapper>
			<Header />
			<LoginFormContents />
		</ProviderWrapper>
	);
};

export default page;
