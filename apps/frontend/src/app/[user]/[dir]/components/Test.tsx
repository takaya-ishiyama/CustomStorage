import { Box, Button } from "@radix-ui/themes";
import React from "react";
import styled from "styled-components";

const Wrapper = styled(Button)`
width: 100%;
`;

export const Test = () => {
	return (
		<div>
			<Wrapper />
		</div>
	);
};
