import { AxiosError } from "axios";
import React from "react";
import {
	UseMutationOptions,
	UseMutationResult,
	useMutation,
} from "react-query";
import { base_uri } from "../backendUri";
import { mutation, query } from "@/infrastructure/graphql/makeSchema";
import { parseCookies, setCookie } from "nookies";

type RegisterResult = {
	id: string;
	username: string;
	accessToken: string;
	refreshToken: string;
};

type RegisterRequest = {
	username: string;
	password: string;
};

type CreateUserProps = {
	options?: UseMutationOptions<
		RegisterResult,
		AxiosError,
		RegisterRequest,
		undefined
	>;
};

type CreateUser = ({
	options,
}: CreateUserProps) => UseMutationResult<
	RegisterResult,
	AxiosError,
	RegisterRequest,
	undefined
>;

export const useResgistUserMutation: CreateUser = ({ options }) => {
	return useMutation(
		async (input: RegisterRequest): Promise<RegisterResult> => {
			const shcema = mutation.register();
			const resp = await fetch(base_uri, {
				method: "POST",
				headers: {
					Accept: "application/json",
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					query: shcema,
					variables: { ...input },
				}),
			});
			if (!resp.ok) {
				throw new Error("Error");
			}
			const {
				data: { createUser },
			} = await resp.json();

			setCookie(null, "accessToken", createUser.accessToken, {
				maxAge: 60 * 60 * 60 /*60min X 60second*/,
			});
			setCookie(null, "refreshToken", createUser.refreshToken, {
				maxAge: 24 * 60 * 60 * 60 /* 24h X 60min X 60second*/,
			});

			return createUser;
		},
		options,
	);
};
