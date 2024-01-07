import { Box, Flex, TextFieldInput } from "@radix-ui/themes";
import React from "react";
import { Controller, FieldValues, UseControllerProps } from "react-hook-form";

type InputProps = React.ComponentProps<typeof TextFieldInput>;
type BoxProps = React.ComponentProps<typeof Box>;

type OmitForInputWithRHF = Omit<InputProps, "value" | "onChange" | "onBlur">;
type OmitForBoxWithRHF = Omit<BoxProps, "textColor">;

type InputWithRHFProps<
	TFieldValues extends FieldValues,
	Props = Record<never, never>,
> = UseControllerProps<TFieldValues> & {
	inputProps?: OmitForInputWithRHF;
	errorProps?: OmitForBoxWithRHF;
	children?: React.ReactNode;
} & Props;

export const InputWithRHF = <TFieldValues extends FieldValues>({
	inputProps,
	errorProps,
	children,
	...rhfProps
}: InputWithRHFProps<TFieldValues>) => (
	<Controller
		{...rhfProps}
		render={({
			field: { onChange, onBlur, value, ref },
			fieldState: { error },
		}) => (
			<Flex>
				<Flex>
					<TextFieldInput
						{...inputProps}
						ref={ref}
						value={value ?? ""}
						onChange={onChange}
						onBlur={onBlur}
					/>
					{children}
				</Flex>
				<Flex>{error?.message != null ? [error.message] : []}</Flex>
			</Flex>
		)}
	/>
);
