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

export default function MyApp() {
	const router = useRouter();
	const handleClickGoHomePage = () => {
		router.push("/home");
	};
	return (
		<>
			<AppBar position="static">
				<Toolbar variant="dense">
					<IconButton
						edge="start"
						color="inherit"
						aria-label="menu"
						sx={{ mr: 2 }}
						onClick={handleClickGoHomePage}
					>
						<MenuOutlined />
					</IconButton>
					<Typography variant="h6" color="inherit" component="div">
						Photos
					</Typography>
				</Toolbar>
			</AppBar>
			<Button variant="contained" onClick={handleClickGoHomePage} />
		</>
	);
}
