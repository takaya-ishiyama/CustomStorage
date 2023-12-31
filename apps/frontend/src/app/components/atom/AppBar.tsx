import { AppBar as MUIAppBar, AppBarTypeMap } from "@mui/material";
import React, { PropsWithChildren } from "react";

type Props = Partial<Omit<AppBarTypeMap<"", "header">, "position">> &
	PropsWithChildren;

export const AppBar: React.FC<Props> = (props) => {
	return (
		<MUIAppBar position="static" {...props}>
			{props.children}
		</MUIAppBar>
	);
};
