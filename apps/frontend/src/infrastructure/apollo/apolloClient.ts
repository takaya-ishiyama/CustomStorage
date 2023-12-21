import { ApolloClient, HttpLink, InMemoryCache } from "@apollo/client";
import { registerApolloClient } from "@apollo/experimental-nextjs-app-support/rsc";
// import { registerApolloClient } from "@apollo/experimental-nextjs-app-support/rsc";

require("dotenv").config({ path: "../../.env" });

const BACKEND_URI = process.env.BACKEND_URI;

export const client = new ApolloClient({
	uri: BACKEND_URI,
	cache: new InMemoryCache(),
});

// export const { getClient } = registerApolloClient(() => {
// 	return new ApolloClient({
// 		cache: new InMemoryCache(),
// 		link: new HttpLink({
// 			uri: BACKEND_URI,
// 			// fetchOptions: { cache: "n" },
// 		}),
// 	});
// });
