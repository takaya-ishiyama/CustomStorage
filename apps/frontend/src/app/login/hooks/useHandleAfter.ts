import { LoginResult } from "@/infrastructure/Query/authorization";
import React from "react";

export const useHandleAfter = () => {
	const onSuccess = React.useCallback((data: LoginResult) => {
		console.log(data);
	}, []);

	const onError = React.useCallback((error: unknown) => {
		console.error(error);
	}, []);

	return {
		onSuccess,
		onError,
	};
};
