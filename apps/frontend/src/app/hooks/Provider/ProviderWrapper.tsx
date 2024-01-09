import React, { PropsWithChildren } from "react";
import { Hydrate, QueryClient, QueryClientProvider } from "react-query";

const queryClient = new QueryClient();

export const ProviderWrapper: React.FC<PropsWithChildren> = ({ children }) => (
	<QueryClientProvider client={queryClient}>
		<Hydrate>{children}</Hydrate>
	</QueryClientProvider>
);
