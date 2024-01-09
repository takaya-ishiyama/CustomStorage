"use client";
import { InputWithRHF } from "@/app/components/molecules/InputWithRFH";
import React from "react";
import { useRegisterForm } from "../hooks/useRegisterForm";
import { useCreateUser } from "../hooks/useCreateUser";
import { Button, Flex } from "@radix-ui/themes";
import { useHandleAfter } from "../hooks/useHandleAfter";

export const FormContents = () => {
	const {
		control,
		formState: { errors, isLoading },
		handleSubmit: handleSubmitWrapper,
	} = useRegisterForm();
	const { handleSuccess } = useHandleAfter();

	const { createUser } = useCreateUser({ onSuccess: handleSuccess });
	const handleSubmit = handleSubmitWrapper(({ username, password }) =>
		createUser({ username, password }),
	);
	return (
		<>
			<InputWithRHF control={control} name={"username"} />
			<InputWithRHF control={control} name={"password"} />
			<Flex>{errors.username?.message}</Flex>
			<Button onClick={handleSubmit}>{"送信"}</Button>
		</>
	);
};
