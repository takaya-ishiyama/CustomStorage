import { useForm } from "react-hook-form";

type User = {
	username: string;
	password: string;
};

const initValue = {
	username: "",
	password: "",
};

export const useRegisterForm = () => {
	const form = useForm<User>({
		defaultValues: initValue,
		reValidateMode: "onChange",
	});
	return form;
};
