import { Button } from "@mui/material";
import React from "react";

export const MainContents = () => {
	const { getFn } = useGet();

	return (
		<div>
			<Button onClick={getFn}>aaa</Button>
		</div>
	);
};

async function getServerSideProps() {
	// Fetch data from external API

	const res = await fetch("http://localhost:8000/json", {
		method: "GET", // or 'POST', 'PUT', etc.
		headers: {
			"Content-Type": "application/json",
			// Add any other custom headers if needed
		},
	});

	const data = await res.json();

	// Pass data to the page via props
	return { props: { data } };
}
const useGet = () => {
	const getFn = async () => {
		const { props } = await getServerSideProps();
		console.log(props.data);
	};
	return { getFn };
};
