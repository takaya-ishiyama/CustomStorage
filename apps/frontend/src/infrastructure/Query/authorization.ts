import { destroyCookie, parseCookies, setCookie } from "nookies";
import { base_uri } from "./backendUri";
import { mutation } from "../graphql/schema";
import {
	UseMutationOptions,
	UseMutationResult,
	useMutation,
} from "react-query";
import { AxiosError } from "axios";

export const login = (
	username: string,
	password: string,
): Promise<Response> => {
	const shcema = mutation.login();
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
	const cookies = parseCookies();
	return fetch(base_uri, {
		method: "POST",
		body: JSON.stringify(cookies.refreshToken),
		headers: {
			Accept: "application/json",
			"Content-Type": "application/json",
		},
	});
};

async function fetchUserByAccessToken(): Promise<Response> {
	const cookies = parseCookies();
	return fetch(base_uri, {
		method: "GET",
		headers: {
			Authorization: `${cookies.accessToken}`,
		},
	});
}

export const GetUser = async (
	username: string,
	password: string,
): Promise<Response | undefined> => {
	const resp = await login(username, password);
	if (resp.ok) {
		const tokenData = await resp.json();
		setCookie(null, "accessToken", tokenData.access, {
			maxAge: 60 * 60 * 60 /*30min X 60second*/,
		});
		setCookie(null, "refreshToken", tokenData.refresh, {
			maxAge: 24 * 60 * 60 * 60 /* 24h X 60min X 60second*/,
		});
	}
	return resp;
};

export const UseTokenGetUser = async () => {
	const cookies = parseCookies();
	if (cookies.accessToken) {
		// アクセストークンがあればユーザー認証
		const resp = await fetchUserByAccessToken();
		return resp;
	}
	if (cookies.refreshToken) {
		// リフレッシュトークンがあればアクセストークンをとってきてユーザー認証
		const resp = await fetchNewToken();
		const tokenData = await resp.json();
		if (resp.ok) {
			setCookie(null, "accessToken", tokenData.access, {
				maxAge: 60 * 60 /*60min X 60second*/,
			});
			setCookie(null, "refreshToken", tokenData.refresh, {
				maxAge: 24 * 60 * 60 /* 24h X 60min X 60second*/,
			});
			const user = await fetchUserByAccessToken();
			return user;
		}
		return;
	}
	// なければ何も返さない
	console.log("error : fetch access token");
	return;
};

type LoginRequest = {
	username: string;
	password: string;
};
type LoginResult = {
	id: string;
	username: string;
	accessToken: string;
	refreshToken: string;
};

type MutationLoginProps = {
	options?: UseMutationOptions<
		LoginResult,
		AxiosError,
		LoginRequest,
		undefined
	>;
};

type MutationLogin = ({
	options,
}: MutationLoginProps) => UseMutationResult<
	LoginResult,
	AxiosError,
	LoginRequest,
	undefined
>;

export const useLogin: MutationLogin = ({ options }) => {
	return useMutation(async (input: LoginRequest): Promise<LoginResult> => {
		const resp = await login(input.username, input.password);
		if (!resp.ok) {
			throw new Error(`HTTP error! Status: ${resp.status}`);
		}

		const {
			data: { login: loginResult },
		} = (await resp.json()) as { data: { login: LoginResult } };

		setCookie(null, "accessToken", loginResult.accessToken, {
			maxAge: 60 * 60 * 60 /*30min X 60second*/,
		});
		setCookie(null, "refreshToken", loginResult.refreshToken, {
			maxAge: 24 * 60 * 60 * 60 /* 24h X 60min X 60second*/,
		});

		return loginResult;
	}, options);
};
