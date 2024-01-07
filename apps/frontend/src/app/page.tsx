import { Box, Button, ThemePanel } from "@radix-ui/themes";
import React from "react";
import Link from "next/link";
import { useRoutes } from "./routes";
import { Bubble } from "./components/organisms/Bubble";
export default function MyApp() {
	const { getLogin, getRegister } = useRoutes();

	return (
		<>
			{/* <ThemePanel /> */}
			<Box>
				<Link href={getLogin()}>
					<Button>ログイン</Button>
				</Link>
			</Box>
			<Box>
				<Link href={getRegister()}>
					<Button>ユーザー登録</Button>
				</Link>
			</Box>
		</>
	);
}
