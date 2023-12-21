import { client } from "@/infrastructure/apollo";
import { gql } from "@apollo/client";
import React from "react";
import { useQuery } from "react-query";

export const useUserQuery = () => {
	const handleQuery = React.useCallback(async () => {
		const data = await client.query({
			query: gql`
              query users {
                user {
                  id
                  username  
                }
              }
            `,
		});
	}, []);

	return {
		handleQuery,
	};
};
