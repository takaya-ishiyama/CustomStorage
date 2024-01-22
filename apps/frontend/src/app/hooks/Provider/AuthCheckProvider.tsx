"use client";
import { useQueryUserWithNewToken } from "@/infrastructure/Query/authorization";
import { useRouter } from "next/navigation";
import React, { PropsWithChildren } from "react";
import { useRoutes } from "../../routes";
import Loading from "../../loading";
import { userAtom } from "../jotai/user/atom";
import { useSetAtom } from "jotai";

export const AuthCheckProvider: React.FC<PropsWithChildren> = ({
	children,
}) => {
	const router = useRouter();
	const setUser = useSetAtom(userAtom);
	const { getHome } = useRoutes();
	const { data, isLoading, isError } = useQueryUserWithNewToken({
		options: {
			retry: 1,
			onSuccess: (data) => {
				if (data === undefined) return;
				setUser({ id: data.id, username: data.username });
				router.replace(getHome(data.id));
			},
			onError: (error) => {
				console.log("error", error);
			},
		},
	});

	if (isLoading) return <Loading />;
	if (data?.id === undefined) return <>Error</>;
	return <>{children}</>;
};
