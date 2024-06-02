import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { kitRoutes } from 'vite-plugin-kit-routes';

export default defineConfig({
	plugins: [
		sveltekit(),
		kitRoutes({
			routes_path: './app/routes',
			generated_file_path: 'app/lib/ROUTES.ts'
		})
	],
	server: {
		fs: {
			deny: ['src'],
			allow: ['app/app.css']
		}
	},
	test: {
		include: ['app/**/*.{test,spec}.{js,ts}']
	},
	optimizeDeps: {
		exclude: ['@urql/svelte']
	}
});
