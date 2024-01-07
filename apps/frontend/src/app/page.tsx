import { Button, ThemePanel } from "@radix-ui/themes";
import React from "react";
import Link from "next/link";
import { useRoutes } from "./routes";
export default function MyApp() {
	const { getLogin } = useRoutes();

	return (
		<>
			{/* <ThemePanel /> */}
			<Link href={getLogin()}>
				<Button>aaaaaa</Button>
			</Link>
			{/* <ProviderWrapper> */}
			{/* <CustomHeader /> */}
			{/* <AuthCheckContents /> */}
			{/* <Button onClick={handleClickGoHomePage}>Go Home Page</Button> */}
			{/* </ProviderWrapper> */}
		</>
	);
}
