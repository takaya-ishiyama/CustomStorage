import { Button } from "@mui/material";
import React from "react";

export const MainContents = () => {
	const { getFn } = useGet();

	// const { handleQuery } = useUserQuery();
	// const { data: queryData } = await getClient().query<Users>({
	// 	query: users,
	// });

	// console.log("kkkksksks", queryData);

	return (
		<div>
			<Button onClick={getFn}>aaa</Button>
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

const getServerSideProps = async () => {
	// Fetch data from external API

	const res = await fetch("http://localhost:8000/graphql", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
			// Add any other custom headers if needed
		},
		body: JSON.stringify(query),
	});

	const data = await res.json();

	return { props: { data } };
};

const useGet = () => {
	const getFn = async () => {
		const { props } = await getServerSideProps();
	};
	return { getFn };
};

type Users = {
	id: string;
	username: string;
	password: string;
};
