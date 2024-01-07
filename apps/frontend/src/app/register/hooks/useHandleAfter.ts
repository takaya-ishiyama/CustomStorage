import { useRoutes } from "@/app/routes";
import { useRouter } from "next/navigation";
import React from "react";

export const useHandleAfter = () => {
	const router = useRouter();
	const { getHome } = useRoutes();

	const onSuccess = React.useCallback(
		(userId: string) => {
			router.push(getHome(userId));
		},
		[getHome, router],
	);

	return {
		onSuccess,
	};
};
