import React from "react";
import { useForm } from "react-hook-form";
import { initValue } from "./validate";

type User = {
	username: string;
	password: string;
};

export const useLoginForm = () => {
	const form = useForm<User>({
		defaultValues: initValue,
		reValidateMode: "onChange",
	});
	return form;
};
