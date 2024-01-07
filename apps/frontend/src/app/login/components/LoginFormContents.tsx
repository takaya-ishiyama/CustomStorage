import { InputWithRHF } from "@/app/components/molecules/InputWithRFH";
import React from "react";
import { useForm } from "react-hook-form";
import { useLoginForm } from "../hooks/useLoginForm";
import { useMutateLoginUser } from "../hooks/useMutateLoginUser";
import { useHandleAfter } from "../hooks/useHandleAfter";
import { Button, Flex } from "@radix-ui/themes";

export const LoginFormContents = () => {
	const {
		control,
		formState: { errors, isLoading },
		handleSubmit: handleSubmitWrapper,
	} = useLoginForm();

	const { onError, onSuccess } = useHandleAfter();

	const {
		handleClickLogin,
		isSuccess,
		isLoading: isSubimit,
	} = useMutateLoginUser({
		onError,
		onSuccess,
	});

	const handleSubmit = handleSubmitWrapper(({ username, password }) =>
		handleClickLogin({ username, password }),
	);

	return (
		<div>
			<InputWithRHF control={control} name={"username"} />
			<InputWithRHF control={control} name={"password"} />
			<Flex>{errors.username?.message}</Flex>
			<Button onClick={handleSubmit}>{"送信"}</Button>
		</div>
	);
};
