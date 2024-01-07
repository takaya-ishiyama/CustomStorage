import React from "react";
import { CustomizedMenus } from "../organisms/CustomizedMenus";
import { useRouter } from "next/navigation";
import { Routes } from "@/app/routes";
import { ContextMenuItem } from "@radix-ui/themes";

export const CustomHeader = () => {
	const router = useRouter();
	const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
	const handleClick = (event: React.MouseEvent<HTMLElement>) => {
		setAnchorEl(event.currentTarget);
	};

	const handleClickGoLoginPage = React.useCallback(
		() => router.push(Routes.login),
		[router],
	);
	const handleClickGoRegister = React.useCallback(
		() => router.push(Routes.register),
		[router],
	);
	return (
		<div>
			{/* <Header onClickMenue={handleClick} />
			<CustomizedMenus anchorEl={anchorEl} setAnchorEl={setAnchorEl}>
				<ContextMenuItem onClick={handleClickGoLoginPage}>
					ログイン
				</ContextMenuItem>
				<ContextMenuItem onClick={() => {}}>ログアウト</ContextMenuItem>
				<ContextMenuItem onClick={handleClickGoRegister}>
					アカウント作成
				</ContextMenuItem>
			</CustomizedMenus> */}
		</div>
	);
};
