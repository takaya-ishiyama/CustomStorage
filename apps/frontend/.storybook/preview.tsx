import type { Preview } from "@storybook/react";

import "@radix-ui/themes/styles.css";
import { Theme } from "@radix-ui/themes";
import React from "react";

const preview: Preview = {
	parameters: {
		actions: { argTypesRegex: "^on[A-Z].*" },
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/i,
			},
		},
		// layout: (Story) => {
		// 	return (
		// 		<Theme
		// 			appearance="light"
		// 			accentColor="teal"
		// 			grayColor="gray"
		// 			radius="large"
		// 		>
		// 			<Story />
		// 		</Theme>
		// 	);
		// },
	},
	decorators: [
		(Story) => {
			return (
				<Theme
					appearance="light"
					accentColor="teal"
					grayColor="gray"
					radius="large"
				>
					<Story />
				</Theme>
			);
		},
	],
};

export default preview;
