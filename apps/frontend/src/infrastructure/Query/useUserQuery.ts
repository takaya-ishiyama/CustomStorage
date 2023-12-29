import { UseQueryOptions, UseQueryResult, useQuery } from "react-query";

type UseUserQuery = (props: Props) => UseQueryResult<User, string[]>;

type Props = {
	options?: Omit<UseQueryOptions<User, string[], User>, "queryKey">;
};

type User = {
	id: string;
	username: string;
};

export const useUserQuery: UseUserQuery = ({ options }) => {
	const query = {
		query: `
	  query {
		getUser(id: "17b5ac0c-1429-469a-8522-053f7bf0f80d") {
		  id
		  username
		}
	  }
	`,
	};
	return useQuery<User, string[]>(
		["user"],
		async () => {
			// FIXME: envから取得できない
			const res = await fetch("http://localhost:8000/graphql", {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify(query),
			});
			if (!res.ok) {
				throw new Error(`HTTP error! Status: ${res.status}`);
			}
			const { data } = await res.json();
			return data.user;
		},
		options,
	);
};
