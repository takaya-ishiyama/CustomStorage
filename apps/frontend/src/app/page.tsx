"use client";
import { Box, Button, ThemePanel } from "@radix-ui/themes";
import React from "react";
import Link from "next/link";
import { useRoutes } from "./routes";
import { Bubble } from "./components/organisms/Bubble";
import { useLogout } from "./hooks/auth/useLogout";
export default function MyApp() {
	const { getLogin, getRegister } = useRoutes();
	const { logout } = useLogout();

	return (
		<>
			{/* <ThemePanel /> */}
			<Box>
				<Link href={getLogin()}>
					<Button>ログイン</Button>
				</Link>
			</Box>
			<Box>
				<Button onClick={logout}>ログアウト</Button>
			</Box>
			<Box>
				<Link href={getRegister()}>
					<Button>ユーザー登録</Button>
				</Link>
			</Box>
		</>
	);
}
