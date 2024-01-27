import type { Meta, StoryObj } from "@storybook/react";
import { FullWithModal } from ".";
import { Box, Button, Dialog } from "@radix-ui/themes";
import { Theme } from "@radix-ui/themes";

import "@radix-ui/themes/styles.css";

//👇 This default export determines where your story goes in the story list
const meta: Meta<typeof FullWithModal> = {
	component: FullWithModal,
};

export default meta;
type Story = StoryObj<typeof FullWithModal>;

export const FirstStory: Story = {
	args: {},
};
