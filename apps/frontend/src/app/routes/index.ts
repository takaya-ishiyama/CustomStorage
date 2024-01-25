export const Routes = {
	top: "/",
	home: "/home",
	login: "/login",
	register: "/register",
} as const;

export const useRoutes = () => {
	const getTop = () => "/";
	const getHome = (userId: string) => `/${userId}/home`;
	const getLogin = () => "/login";
	const getRegister = () => "/register";
	return {
		getTop,
		getHome,
		getLogin,
		getRegister,
	};
};

export const getRoutes = {
	top: () => "/",
	home: (userId: string) => `/${userId}/home`,
	login: () => "/login",
	register: () => "/register",
};
