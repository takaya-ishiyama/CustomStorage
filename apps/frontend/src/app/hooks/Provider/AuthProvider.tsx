"use client";

import { useAtom } from "jotai";
import React, { PropsWithChildren } from "react";
import { userAtom } from "../jotai/user/atom";
import Login from "@/app/login/page";
import { useQueryUserWithNewToken } from "@/infrastructure/Query/authorization";
import { Loading } from "@/app/components/molecules/Loading";

export const AuthProvider: React.FC<PropsWithChildren> = ({ children }) => {
	const [isVisible, setIsVisible] = React.useState(false);
	const [user, setUser] = useAtom(userAtom);
	const { data, refetch, isLoading, isError } = useQueryUserWithNewToken({
		options: {
			enabled: false,
			retry: 3,
		},
	});
	React.useEffect(() => {
		if (user.id) {
			setIsVisible(true);
			return;
		}
		refetch().then(({ data }) => {
			setIsVisible(true);
			if (!data) return;
			setUser({ id: data.id, username: data.username });
		});
	}, [refetch, setUser, user.id, setIsVisible]);

	if (!isVisible) return <Loading />;
	return <>{user.id ? <>{children}</> : <Login />}</>;
};
