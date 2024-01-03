import { Box, Input } from "@mui/material";
import React from "react";
import { Controller, FieldValues, UseControllerProps } from "react-hook-form";

type InputProps = React.ComponentProps<typeof Input>;
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
			<Box sx={{ display: "flex" }}>
				<Box sx={{ display: "flex" }}>
					<Input
						{...inputProps}
						ref={ref}
						value={value ?? ""}
						onChange={onChange}
						onBlur={onBlur}
					/>
					{children}
				</Box>
				<Box>{error?.message != null ? [error.message] : []}</Box>
			</Box>
		)}
	/>
);
