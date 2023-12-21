"use client";

import { client } from "@/infrastructure/apollo";
import { ApolloProvider } from "@apollo/client";
import { Button } from "@mui/material";
import { useRouter } from "next/navigation";

export default function MyApp() {
	const router = useRouter();
	const handleClickGoHomePage = () => {
		router.push("/home");
	};
	return (
		<>
			<ApolloProvider client={client}>
				<div>
					<Button variant="contained" onClick={handleClickGoHomePage} />
				</div>
			</ApolloProvider>
		</>
	);
}
