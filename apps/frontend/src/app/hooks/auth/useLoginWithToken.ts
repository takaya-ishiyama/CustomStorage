"use client";
import React from "react";
import { userAtom } from "../jotai/user/atom";
import { useSetAtom } from "jotai";
import { useQueryUserWithNewToken } from "@/infrastructure/Query/authorization";

type Props = {
	onSuccess?: () => void;
	onError?: () => void;
};

export const useLoginWithToken = ({ onSuccess, onError }: Props) => {
	const setUser = useSetAtom(userAtom);
	const { data, isLoading, isError, refetch } = useQueryUserWithNewToken({
		options: {
			retry: 1,
			enabled: false,
			onSettled: (data) => {
				if (data === undefined) return;
				setUser({ id: data.id, username: data.username });
				onSuccess?.();
			},
			onError: (error) => {
				console.log("error", error);
				onError?.();
			},
		},
	});
	const loginFn = React.useCallback(async () => {
		await refetch();
	}, [refetch]);

	return { loginFn, isLoading, isError, data };
};
