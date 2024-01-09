import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
	overwrite: true,
	schema: "../backend/graphql/schema.gql",
	// documents: "src/**/*.tsx",
	generates: {
		"src/infrastructure/graphql/": {
			preset: "client",
			plugins: [],
		},
		"./graphql.schema.json": {
			plugins: ["introspection"],
		},
	},
};

export default config;
