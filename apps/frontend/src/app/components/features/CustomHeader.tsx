import React from "react";
import { Header } from "../organisms/Header";
import { CustomizedMenus } from "../organisms/CustomizedMenus";
import { MenuItem } from "@mui/material";
import { useRouter } from "next/navigation";
import { Routes } from "@/app/routes";

export const CustomHeader = () => {
	const router = useRouter();
	const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
	const handleClick = (event: React.MouseEvent<HTMLElement>) => {
		setAnchorEl(event.currentTarget);
	};

	const handleClickGoLoginPage = () => router.push(Routes.login);
	return (
		<div>
			<Header onClickMenue={handleClick} />
			<CustomizedMenus anchorEl={anchorEl} setAnchorEl={setAnchorEl}>
				<MenuItem onClick={handleClickGoLoginPage}>ログイン</MenuItem>
				<MenuItem onClick={() => {}}>ログアウト</MenuItem>
			</CustomizedMenus>
		</div>
	);
};
