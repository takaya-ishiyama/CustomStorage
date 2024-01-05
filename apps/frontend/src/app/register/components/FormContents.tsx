import { InputWithRHF } from "@/app/components/molecules/InputWithRFH";
import { Box, Button } from "@mui/material";
import React from "react";
import { useRegisterForm } from "../hooks/useRegisterForm";
import { useCreateUser } from "../hooks/useCreateUser";
import { useRouter } from "next/navigation";
import { Routes } from "@/app/routes";

export const FormContents = () => {
	const router = useRouter();
	const {
		control,
		formState: { errors, isLoading },
		handleSubmit: handleSubmitWrapper,
	} = useRegisterForm();

	const onSuccess = React.useCallback(() => {
		router.push(Routes.home);
	}, [router]);
	const { createUser } = useCreateUser({ onSuccess });
	const handleSubmit = handleSubmitWrapper(({ username, password }) =>
		createUser({ username, password }),
	);
	return (
		<>
			<InputWithRHF control={control} name={"username"} />
			<InputWithRHF control={control} name={"password"} />
			<Box>{errors.username?.message}</Box>
			<Button onClick={handleSubmit}>{"送信"}</Button>
		</>
	);
};
