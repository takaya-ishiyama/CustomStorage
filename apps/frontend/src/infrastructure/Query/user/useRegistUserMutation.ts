import { AxiosError } from "axios";
import React from "react";
import {
	UseMutationOptions,
	UseMutationResult,
	useMutation,
} from "react-query";
import { base_uri } from "../backendUri";
import { parseCookies, setCookie } from "nookies";
import {
	CreateUser,
	MutationCreateUserArgs,
} from "@/infrastructure/graphql/graphql";
import { mutation } from "@/infrastructure/graphql/requestBody";

type CreateUserProps = {
	options?: UseMutationOptions<
		CreateUser,
		AxiosError,
		MutationCreateUserArgs,
		undefined
	>;
};

type CreateUserQuery = ({
	options,
}: CreateUserProps) => UseMutationResult<
	CreateUser,
	AxiosError,
	MutationCreateUserArgs,
	undefined
>;

export const useResgistUserMutation: CreateUserQuery = ({ options }) => {
	return useMutation(
		async (input: MutationCreateUserArgs): Promise<CreateUser> => {
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
