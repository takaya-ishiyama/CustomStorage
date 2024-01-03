import React from "react";
import { useForm } from "react-hook-form";

type User = {
	name: string;
	password: string;
};

export const LoginForm = () => {
	const form = useForm<User>({
		reValidateMode: "onChange",
	});
	return form;
};
