import { userAtom } from "@/app/hooks/jotai/user/atom";
import { LoginResult, useLogin } from "@/infrastructure/Query/authorization";
import { AxiosError } from "axios";
import { useSetAtom, useStore } from "jotai";
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
	const setUserAtom = useSetAtom(userAtom);
	const { mutate, isSuccess, isLoading } = useLogin({});

	const handleClickLogin = React.useCallback(
		({ username, password }: LoginInputProps) => {
			mutate(
				{ username, password },
				{
					onSuccess: (data) => {
						setUserAtom({ id: data.id, username: data.username });
						onSuccess?.(data);
					},
					onError: (error) => {
						onError?.(error);
					},
				},
			);
		},
		[mutate, onError, onSuccess, setUserAtom],
	);

	return {
		handleClickLogin,
		isSuccess,
		isLoading,
	};
};
