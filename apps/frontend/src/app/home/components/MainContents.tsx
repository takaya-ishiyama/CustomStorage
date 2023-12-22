import { useUserQuery } from "@/infrastructure/Query/useUserQuery";
import { Button } from "@mui/material";
import React from "react";

export const MainContents = () => {
	const { data, isLoading } = useUserQuery({});

	return (
		<div>
			<Button onClick={() => {}}>aaa</Button>
		</div>
	);
};

const query = {
	query: `
	  query {
		users {
		  id
		  username  
		}
	  }  
	`,
};
