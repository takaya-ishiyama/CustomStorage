import { Login } from "@/infrastructure/graphql/graphql";
import { useRouter } from "next/navigation";
import React from "react";

export const useHandleAfter = () => {
	const router = useRouter();
	const onSuccess = React.useCallback(
		(data: Login) => {
			router.push("/home");
		},
		[router],
	);

	const onError = React.useCallback((error: unknown) => {
		console.error(error);
	}, []);

	return {
		onSuccess,
		onError,
	};
};
