import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { kitRoutes } from 'vite-plugin-kit-routes';
import type { KIT_ROUTES } from './app/lib/ROUTES';

export default defineConfig({
	plugins: [
		sveltekit(),
		kitRoutes<KIT_ROUTES>({
			routes_path: 'app/routes',
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
