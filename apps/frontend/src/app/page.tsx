"use client";

import { MenuBook, MenuOpen, MenuOutlined } from "@mui/icons-material";
import {
	AppBar,
	Button,
	Icon,
	IconButton,
	Menu,
	Toolbar,
	Typography,
} from "@mui/material";
import { useRouter } from "next/navigation";
import { Header } from "./components/organisms/Header";
import { base_uri } from "@/infrastructure/Query/backendUri";

export default function MyApp() {
	const router = useRouter();
	const handleClickGoHomePage = () => {
		router.push("/home");
	};

	return (
		<>
			<Header />
			<Button onClick={handleClickGoHomePage}>Go Home Page</Button>
		</>
	);
}
