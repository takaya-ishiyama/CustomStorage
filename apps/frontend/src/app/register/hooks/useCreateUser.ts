import { userAtom } from "@/app/hooks/jotai/user/atom";
import { useResgistUserMutation } from "@/infrastructure/Query/user/useRegistUserMutation";
import { useSetAtom } from "jotai";
import React from "react";

type Props = {
	onSuccess?: (data: unknown) => void;
	onError?: (error: unknown) => void;
};

type CreateUserProps = {
	username: string;
	password: string;
};

export const useCreateUser = ({ onSuccess }: Props) => {
	const setUserAtom = useSetAtom(userAtom);
	const { mutate } = useResgistUserMutation({
		options: {
			onSuccess: (data) => {
				setUserAtom({ id: data.id, username: data.username });
				onSuccess?.(data);
			},
		},
	});
	const createUser = React.useCallback(
		({ username, password }: CreateUserProps) => {
			mutate({ username, password });
		},
		[mutate],
	);

	return {
		createUser,
	};
};
