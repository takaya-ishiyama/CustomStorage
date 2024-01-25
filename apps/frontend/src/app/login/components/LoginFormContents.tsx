"use client";
import { InputWithRHF } from "@/app/components/molecules/InputWithRFH";
import React from "react";
import { useLoginForm } from "../hooks/useLoginForm";
import { useMutateLoginUser } from "../hooks/useMutateLoginUser";
import { useHandleAfter } from "../hooks/useHandleAfter";
import { Button, Flex } from "@radix-ui/themes";
import { useQueryUserWithNewToken } from "@/infrastructure/Query/authorization";
import { useRouter } from "next/navigation";
import { getRoutes } from "@/app/routes";

export const LoginFormContents = () => {
	const router = useRouter();
	const { data, isLoading: queryLoading } = useQueryUserWithNewToken({
		options: {
			retry: 3,
			onSuccess: (data) => {
				console.log("data", data);
				router.push(getRoutes.home(data.id));
			},
			onError: (error) => {
				console.log("error", error);
			},
		},
	});

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

	if (queryLoading || isLoading) return <div>loading</div>;

	return (
		<div>
			<InputWithRHF control={control} name={"username"} />
			<InputWithRHF control={control} name={"password"} />
			<Flex>{errors.username?.message}</Flex>
			<Button onClick={handleSubmit}>{"送信"}</Button>
		</div>
	);
};
