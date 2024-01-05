"use client";

import React from "react";
import { ProviderWrapper } from "../hooks/Provider/ProviderWrapper";
import { CustomHeader } from "../components/features/CustomHeader";
import { FormContents } from "./components/FormContents";

const Register = () => {
	return (
		<ProviderWrapper>
			<CustomHeader />
			<FormContents />
		</ProviderWrapper>
	);
};

export default Register;
