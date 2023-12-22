import React, { PropsWithChildren } from "react";
import { QueryClient, QueryClientProvider } from "react-query";

const queryClient = new QueryClient();

export const ProviderWrapper: React.FC<PropsWithChildren> = ({ children }) => (
	<QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
);
