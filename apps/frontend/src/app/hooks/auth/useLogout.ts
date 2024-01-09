import { useRouter } from "next/navigation";
import { destroyCookie } from "nookies";
import React from "react";

export const useLogout = () => {
	const route = useRouter();

	const logout = React.useCallback(() => {
		destroyCookie(null, "accessToken");
		destroyCookie(null, "refreshToken");
		route.replace("/");
	}, [route]);
	return { logout };
};
