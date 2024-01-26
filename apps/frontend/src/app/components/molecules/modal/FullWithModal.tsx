import { Box, Button, Dialog } from "@radix-ui/themes";
import React, { PropsWithChildren } from "react";

type Props = React.ComponentProps<typeof Dialog.Root> & PropsWithChildren;

export const FullWithModal: React.FC<Props> = ({ children }) => {
	return (
		<Dialog.Root>
			<Dialog.Trigger>
				<Button>Edit profile</Button>
			</Dialog.Trigger>

			<Dialog.Content style={{ maxWidth: 450 }}>
				<Dialog.Title>
					<Box>Edit profile</Box>
				</Dialog.Title>
				<Dialog.Description size="2" mb="4">
					<Box>Make changes to your profile.</Box>
				</Dialog.Description>

				<Dialog.Close>
					<Button variant="soft" color="gray">
						Cancel
					</Button>
				</Dialog.Close>
				<Dialog.Close>
					<Button>Save</Button>
				</Dialog.Close>
			</Dialog.Content>
		</Dialog.Root>
	);
};
