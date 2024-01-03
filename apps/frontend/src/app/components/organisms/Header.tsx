import { MenuOutlined } from "@mui/icons-material";
import { Toolbar, Typography, IconButton, Box, Button } from "@mui/material";
import React from "react";
import { AppBar } from "../atom/AppBar";
import { useRouter } from "next/navigation";

type Props = {
	onClickMenue?: React.MouseEventHandler<HTMLButtonElement> | undefined;
};

export const Header: React.FC<Props> = (props) => {
	const router = useRouter();
	return (
		<AppBar>
			<Toolbar variant="dense">
				<Typography
					variant="h6"
					color="inherit"
					component="div"
					onClick={() => router.push("")}
				>
					CustomStorage
				</Typography>
				<IconButton
					edge="end"
					color="inherit"
					aria-label="menu"
					sx={{ mr: 2 }}
					onClick={props.onClickMenue}
					style={{ marginLeft: "auto" }}
				>
					<MenuOutlined />
				</IconButton>
			</Toolbar>
		</AppBar>
	);
};
