import { destroyCookie, parseCookies, setCookie } from "nookies";
import { base_uri } from "./backendUri";

const fetchToken = (username: string, password: string): Promise<Response> => {
	return fetch(base_uri, {
		method: "POST",
		// body: JSON.stringify({ username, password }),
		headers: {
			Accept: "application/json",
			"Content-Type": "application/json",
		},
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

async function fetchUser(): Promise<Response> {
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
	const resp = await fetchToken(username, password);
	if (resp.ok) {
		const tokenData = await resp.json();
		setCookie(null, "accessToken", tokenData.access, {
			maxAge: 60 * 60 * 60 /*30min X 60second*/,
		});
		setCookie(null, "refreshToken", tokenData.refresh, {
			maxAge: 24 * 60 * 60 * 60 /* 24h X 60min X 60second*/,
		});
		const user = await fetchUser();
		return user;
	}
};

export const UseTokenGetUser = async () => {
	const cookies = parseCookies();
	if (cookies.accessToken) {
		// アクセストークンがあればユーザー認証
		const resp = await fetchUser();
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
			const user = await fetchUser();
			return user;
		}
		return;
	}
	// なければ何も返さない
	console.log("error : fetch access token");
	return;
};
