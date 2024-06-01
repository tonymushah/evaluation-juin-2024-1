import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
	schema: './app/schemas/admin.graphqls',
	documents: [
		'app/lib/admin/**/*.svelte',
		'app/lib/admin/**/*.ts',
		'app/routes/admin/**/*.svelte',
		'app/routes/admin/**/*.ts'
	],
	ignoreNoDocuments: true, // for better experience with the watcher
	generates: {
		'app/lib/admin/gql/': {
			preset: 'client',
			config: {
				useTypeImports: true
			},
			plugins: []
		}
	}
};

export default config;
