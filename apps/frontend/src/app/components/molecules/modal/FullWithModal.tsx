import { Box, Button, Card, Dialog, Flex } from "@radix-ui/themes";
import React, { PropsWithChildren } from "react";

type Props = { isOpen: boolean } & PropsWithChildren;

export const FullWithModal = ({ children, isOpen }: Props) => {
	return <>{isOpen && <Card>aaa</Card>}</>;
};
