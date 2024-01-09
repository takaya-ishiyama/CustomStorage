import { atom } from "jotai";

type UserAtom = {
	id: string | undefined;
	username: string | undefined;
};

export const userAtom = atom<UserAtom>({
	id: undefined,
	username: undefined,
});
