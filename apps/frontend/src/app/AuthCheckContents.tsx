import { useQueryUserWithNewToken } from "@/infrastructure/Query/authorization";
import { useRouter } from "next/navigation";
import React from "react";

export const AuthCheckContents = () => {
	const router = useRouter();
	const { data, isLoading } = useQueryUserWithNewToken();

	React.useEffect(() => {
		console.log("aaaaaaaaa", data);
		if (data?.id !== undefined && data?.id !== null) {
			router.push("/home");
		}
	}, [data]);
	if (isLoading) {
		return <div>Loading...</div>;
	}
	return <div>AuthCheckContents</div>;
};
