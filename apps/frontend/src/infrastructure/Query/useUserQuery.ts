import { UseQueryOptions, useQuery } from "react-query";
import { BACKEND_URI } from "../URI";

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
		user {
		  id
		  username
		}
	  }
	`,
	};
	return useQuery(
		["user"],
		async () => {
			if (BACKEND_URI === undefined)
				throw new Error("BACKEND_URI is undefined");
			const res = await fetch(BACKEND_URI, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify(query),
			});

			const { data } = await res.json();
			return data.user as User;
		},
		options,
	);
};
