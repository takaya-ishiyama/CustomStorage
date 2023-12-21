import { UseQueryOptions, useQuery } from "react-query";

type Props = {
	options?: UseQueryOptions<User, undefined, undefined, string[]>;
};

type User = {
	id: string;
	username: string;
};

export const useUserQuery = ({ options }: Props) => {
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
	return useQuery(
		["user"],
		async () => {
			const res = await fetch("http://localhost:8000/graphql", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
					// Add any other custom headers if needed
				},
				body: JSON.stringify(query),
			});

			const { data } = await res.json();
			return data.users as User;
		},
		options,
	);
};
