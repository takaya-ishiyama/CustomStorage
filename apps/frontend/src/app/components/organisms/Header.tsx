import { MenuOutlined } from "@mui/icons-material";
import { Toolbar, Typography, IconButton } from "@mui/material";
import React from "react";
import { AppBar } from "../atom/AppBar";

type Props = {
	onClickMenue?: () => void;
};

export const Header: React.FC<Props> = (props) => {
	return (
		<AppBar>
			<Toolbar variant="dense">
				<Typography variant="h6" color="inherit" component="div">
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
