import { CLIENT_TOKEN_KEY } from '$lib/equipe/utils/tokenKey';
import { redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { route } from '$lib/ROUTES';

export const actions = {
	default: async ({ cookies }) => {
		cookies.delete(CLIENT_TOKEN_KEY, { path: '' });
		return redirect(300, route('/equipe/login'));
	}
} satisfies Actions;