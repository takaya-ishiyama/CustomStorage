import * as React from "react";
import { Button } from "@radix-ui/themes";
import * as Toast from "@radix-ui/react-toast";

type Props = {
	open: boolean;
	onOpenChange: (open: boolean) => void;
};

export const Notification: React.FC<Props> = ({ open, onOpenChange }) => {
	const eventDateRef = React.useRef(new Date());
	const timerRef = React.useRef(0);

	React.useEffect(() => {
		return () => clearTimeout(timerRef.current);
	}, []);

	return (
		<Toast.Provider swipeDirection="right">
			<Toast.Root className="ToastRoot" open={open} onOpenChange={onOpenChange}>
				<Toast.Title className="ToastTitle">Scheduled: Catch up</Toast.Title>
				<Toast.Description asChild>
					<time
						className="ToastDescription"
						dateTime={eventDateRef.current.toISOString()}
					>
						{}
					</time>
				</Toast.Description>
				<Toast.Action
					className="ToastAction"
					asChild
					altText="Goto schedule to undo"
				>
					<Button className="Button small green">Undo</Button>
				</Toast.Action>
			</Toast.Root>
			<Toast.Viewport className="ToastViewport" />
		</Toast.Provider>
	);
};
