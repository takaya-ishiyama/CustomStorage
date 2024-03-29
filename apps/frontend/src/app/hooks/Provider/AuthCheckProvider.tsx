"use client";
import { useQueryUserWithNewToken } from "@/infrastructure/Query/authorization";
import { useRouter } from "next/navigation";
import React, { PropsWithChildren } from "react";
import { getRoutes, useRoutes } from "../../routes";
import Loading from "../../loading";
import { userAtom } from "../jotai/user/atom";
import { useSetAtom } from "jotai";
import { Box } from "@radix-ui/themes";
import Link from "next/link";

export const AuthCheckProvider: React.FC<PropsWithChildren> = ({
	children,
}) => {
	const setUser = useSetAtom(userAtom);
	const router = useRouter();
	const { data, isLoading, isError } = useQueryUserWithNewToken({
		options: {
			retry: 1,
			onSuccess: (data) => {
				if (data === undefined) return;
				setUser({ id: data.id, username: data.username });
			},
			onError: (error) => {
				console.log("error", error);
			},
		},
	});

	if (isLoading) return <Loading />;
	if (data?.id === undefined)
		return (
			<>
				<Box>Error</Box>
				<Link href={getRoutes.login()}>ログインへ</Link>
			</>
		);
	return <>{children}</>;
};
