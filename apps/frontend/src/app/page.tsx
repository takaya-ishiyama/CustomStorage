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
import { CustomHeader } from "./components/features/CustomHeader";
import React from "react";
import { ProviderWrapper } from "./hooks/ProviderWrapper";
import { AuthCheckContents } from "./AuthCheckContents";

export default function MyApp() {
	const router = useRouter();
	const handleClickGoHomePage = () => {
		router.push("/home");
	};

	React.useEffect(() => {}, []);

	return (
		<>
			<ProviderWrapper>
				<CustomHeader />
				<AuthCheckContents />
				<Button onClick={handleClickGoHomePage}>Go Home Page</Button>
			</ProviderWrapper>
		</>
	);
}
