import { useRoutes } from "@/app/routes";
import { Login } from "@/infrastructure/graphql/graphql";
import { useRouter } from "next/navigation";
import React from "react";

export const useHandleAfter = () => {
	const router = useRouter();
	const { getHome } = useRoutes();
	const onSuccess = React.useCallback(
		(data: Login) => {
			router.replace(getHome(data.id));
		},
		[getHome, router],
	);

	const onError = React.useCallback((error: unknown) => {
		console.error(error);
	}, []);

	return {
		onSuccess,
		onError,
	};
};
