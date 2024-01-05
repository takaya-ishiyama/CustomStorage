import { InputWithRHF } from "@/app/components/molecules/InputWithRFH";
import { Box, Button } from "@mui/material";
import React from "react";
import { useRegisterForm } from "../hooks/useRegisterForm";

export const FormContents = () => {
	const {
		control,
		formState: { errors, isLoading },
		handleSubmit: handleSubmitWrapper,
	} = useRegisterForm();
	return (
		<>
			<InputWithRHF control={control} name={"username"} />
			<InputWithRHF control={control} name={"password"} />
			<Box>{errors.username?.message}</Box>
			<Button onClick={() => {}}>{"送信"}</Button>
		</>
	);
};
