import { InputWithRHF } from "@/app/components/molecules/InputWithRFH";
import React from "react";
import { LoginForm } from "../hooks/LoginForm";

export const LoginFormContents = () => {
	const {
		control,
		formState: { errors, isLoading },
	} = LoginForm();
	return (
		<div>
			<InputWithRHF control={control} name={"name"} />
		</div>
	);
};
