import { Flex } from "@radix-ui/themes";

export const Loading: React.FC = () => {
	return (
		<>
			<Flex
				style={{
					position: "absolute",
					top: 0,
					left: 0,
					width: "100%",
					height: "100%",
					backgroundColor: "rgba(128, 128, 128, 0.3)",
					zIndex: 9999,
				}}
			/>

			<Flex
				style={{
					position: "absolute",
					top: "50%",
					left: "50%",
					transform: "translate(-50%, -50%)",
					zIndex: 10000,
				}}
			>
				<div>Loading...</div>
			</Flex>
		</>
	);
};
