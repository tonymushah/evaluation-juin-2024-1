import type { CodegenConfig } from '@graphql-codegen/cli';

const BASE = 'equipe';

const GEN = `app/lib/${BASE}/gql/`;

const config: CodegenConfig = {
	schema: `./app/schemas/${BASE}.graphqls`,
	documents: [
		`app/lib/${BASE}/**/*.svelte`,
		`app/lib/${BASE}/**/*.ts`,
		`app/routes/${BASE}/**/*.svelte`,
		`app/routes/${BASE}/**/*.ts`
	],
	ignoreNoDocuments: true, // for better experience with the watcher
	generates: {
		GEN: {
			preset: 'client',
			config: {
				useTypeImports: true
			},
			plugins: []
		}
	}
};

export default config;
