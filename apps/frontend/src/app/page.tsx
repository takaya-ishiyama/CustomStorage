"use client";
import { Box, Button, ThemePanel } from "@radix-ui/themes";
import React from "react";
import Link from "next/link";
import { useRoutes } from "./routes";
import { Bubble } from "./components/organisms/Bubble";
import { useLogout } from "./hooks/auth/useLogout";
import { useLoginWithToken } from "./hooks/auth/useLoginWithToken";
import { LoginContents } from "./LoginContents";
import { ProviderWrapper } from "./hooks/Provider/ProviderWrapper";

export default function MyApp() {
	const { getRegister } = useRoutes();
	const { logout } = useLogout();

	return (
		<>
			{/* <ThemePanel /> */}
			<ProviderWrapper>
				<LoginContents />
				<Box>
					<Button onClick={logout}>ログアウト</Button>
				</Box>
				<Box>
					<Link href={getRegister()}>
						<Button>ユーザー登録</Button>
					</Link>
				</Box>
			</ProviderWrapper>
		</>
	);
}
