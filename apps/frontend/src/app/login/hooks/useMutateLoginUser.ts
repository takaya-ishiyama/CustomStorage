import { LoginResult, useLogin } from "@/infrastructure/Query/authorization";
import { AxiosError } from "axios";
import React from "react";

type Props = {
	onSuccess?: (data: LoginResult) => void;
	onError?: (error: AxiosError<unknown> | undefined) => void;
};

type LoginInputProps = {
	username: string;
	password: string;
};

export const useMutateLoginUser = ({ onSuccess, onError }: Props) => {
	const { mutate, data, isSuccess, isLoading } = useLogin({});

	const handleClickLogin = React.useCallback(
		({ username, password }: LoginInputProps) => {
			mutate(
				{ username, password },
				{
					onSuccess: (data) => {
						onSuccess?.(data);
					},
					onError: (error) => {
						onError?.(error);
					},
				},
			);
		},
		[mutate, onError, onSuccess],
	);

	return {
		handleClickLogin,
		data,
		isSuccess,
		isLoading,
	};
};
