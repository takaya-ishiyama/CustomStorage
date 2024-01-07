import { destroyCookie, parseCookies, setCookie } from "nookies";
import { base_uri } from "./backendUri";
import { mutation, query } from "../graphql/requestBody";
import {
	UseMutationOptions,
	UseMutationResult,
	UseQueryOptions,
	UseQueryResult,
	useMutation,
	useQuery,
} from "react-query";
import { AxiosError } from "axios";
import { useSetAtom } from "jotai";
import { Login, QueryLoginArgs } from "../graphql/graphql";

export const login = (
	username: string,
	password: string,
): Promise<Response> => {
	const shcema = query.login();
	return fetch(base_uri, {
		method: "POST",
		headers: {
			Accept: "application/json",
			"Content-Type": "application/json",
		},
		body: JSON.stringify({ query: shcema, variables: { username, password } }),
	});
};

const fetchNewToken = (): Promise<Response> => {
	const shcema = query.get_new_token();
	const cookies = parseCookies();
	return fetch(base_uri, {
		method: "POST",
		headers: {
			Accept: "application/json",
			"Content-Type": "application/json",
			Authorization: cookies.refreshToken,
		},
		body: JSON.stringify({ query: shcema }),
	});
};

async function fetchUserByAccessToken(): Promise<Response> {
	const shcema = query.login_with_token();
	const cookies = parseCookies();
	return fetch(base_uri, {
		method: "POST",
		headers: {
			Accept: "application/json",
			"Content-Type": "application/json",
			Authorization: cookies.accessToken,
		},
		body: JSON.stringify({ query: shcema }),
	});
}

type MutationLoginProps = {
	options?: UseMutationOptions<Login, AxiosError, QueryLoginArgs, undefined>;
};

type MutationLogin = ({
	options,
}: MutationLoginProps) => UseMutationResult<
	Login,
	AxiosError,
	QueryLoginArgs,
	undefined
>;

export const useLogin: MutationLogin = ({ options }) => {
	return useMutation(async (input: QueryLoginArgs): Promise<Login> => {
		const resp = await login(input.username, input.password);
		if (!resp.ok) {
			throw new Error(`HTTP error! Status: ${resp.status}`);
		}

		const {
			data: { login: loginResult },
		} = (await resp.json()) as { data: { login: Login } };

		if (
			loginResult.session.accessToken === null ||
			loginResult.session.accessToken === undefined ||
			loginResult.session.refreshToken === null ||
			loginResult.session.refreshToken === undefined
		)
			throw new Error("error: login failed");

		setCookie(null, "accessToken", loginResult.session.accessToken, {
			maxAge: 60 * 60 * 60 /*60min X 60second*/,
		});
		setCookie(null, "refreshToken", loginResult.session.refreshToken, {
			maxAge: 24 * 60 * 60 * 60 /* 24h X 60min X 60second*/,
		});

		return loginResult;
	}, options);
};

export const useQueryUserWithNewToken = ({
	options,
}: {
	options?: Omit<UseQueryOptions<Login, AxiosError>, "queryKey">;
}): UseQueryResult<Login, AxiosError> => {
	const cookies = parseCookies();
	return useQuery(
		["user"],
		async () => {
			if (cookies.accessToken) {
				const respByaccessToken = await fetchUserByAccessToken();
				if (!respByaccessToken.ok)
					throw new Error(`HTTP error! Status: ${respByaccessToken.status}`);

				const {
					data: { login: loginResult },
				} = (await respByaccessToken.json()) as {
					data: { login: Login };
				};
				return loginResult;
			}

			if (cookies.refreshToken) {
				console.log("refreshToken");
				const resp = await fetchNewToken();
				const {
					data: { getNewToken },
				} = await resp.json();
				if (resp.ok) {
					setCookie(null, "accessToken", getNewToken.accessToken, {
						maxAge: 60 * 60 /*60min X 60second*/,
					});
					setCookie(null, "refreshToken", getNewToken.refreshToken, {
						maxAge: 24 * 60 * 60 /* 24h X 60min X 60second*/,
					});
				}

				const respByaccessToken = await fetchUserByAccessToken();
				const {
					data: { login: loginResult },
				} = (await respByaccessToken.json()) as {
					data: { login: Login };
				};
				return loginResult;
			}
			throw new Error("error: expired fetch token");
		},
		options,
	);
};
