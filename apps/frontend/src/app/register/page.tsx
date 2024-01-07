"use client";

import React from "react";
import { ProviderWrapper } from "../hooks/Provider/ProviderWrapper";
import { CustomHeader } from "../components/features/CustomHeader";
import { FormContents } from "./components/FormContents";
import { Box, Text } from "@radix-ui/themes";

const Register = () => {
	return (
		<ProviderWrapper>
			{/* <CustomHeader /> */}
			<Text>ユーザー登録</Text>
			<FormContents />
		</ProviderWrapper>
	);
};

export default Register;
