"use client";

import React from "react";
import { ProviderWrapper } from "../hooks/Provider/ProviderWrapper";
import { CustomHeader } from "../components/features/CustomHeader";
import { FormContents } from "./components/FormContents";
import { Flex } from "@radix-ui/themes";

const Register = () => {
	return (
		<>
			{/* <CustomHeader /> */}
			<Flex>ユーザー登録</Flex>
			<FormContents />
		</>
	);
};

export default Register;
