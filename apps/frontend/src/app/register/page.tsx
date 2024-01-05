"use client";

import React from "react";
import { ProviderWrapper } from "../hooks/Provider/ProviderWrapper";
import { CustomHeader } from "../components/features/CustomHeader";
import { FormContents } from "./components/FormContents";
import { Box } from "@mui/material";

const Register = () => {
	return (
		<ProviderWrapper>
			<CustomHeader />
			<Box>ユーザー登録</Box>
			<FormContents />
		</ProviderWrapper>
	);
};

export default Register;
