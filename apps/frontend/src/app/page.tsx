"use client";

import { Button } from "@mui/material";
import { useRouter } from "next/navigation";

export default function MyApp() {
	const router = useRouter();
	const handleClickGoHomePage = () => {
		router.push("/home");
	};
	return (
		<>
			<div>
				<Button variant="contained" onClick={handleClickGoHomePage} />
			</div>
		</>
	);
}
