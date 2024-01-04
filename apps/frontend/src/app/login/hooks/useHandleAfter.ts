import { LoginResult } from "@/infrastructure/Query/authorization";
import { useRouter } from "next/navigation";
import React from "react";

export const useHandleAfter = () => {
	const router = useRouter();
	const onSuccess = React.useCallback(
		(data: LoginResult) => {
			router.push("/home");
			console.log(data);
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
