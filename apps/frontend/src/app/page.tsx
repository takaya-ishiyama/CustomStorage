import { Button, ThemePanel } from "@radix-ui/themes";
import { useRouter } from "next/navigation";
import React from "react";
export default function MyApp() {
	// const router = useRouter();
	// const handleClickGoHomePage = () => {
	// 	router.push("/home");
	// };

	return (
		<>
			<ThemePanel />
			<Button>aaaaaa</Button>
			{/* <ProviderWrapper> */}
			{/* <CustomHeader /> */}
			{/* <AuthCheckContents /> */}
			{/* <Button onClick={handleClickGoHomePage}>Go Home Page</Button> */}
			{/* </ProviderWrapper> */}
		</>
	);
}
