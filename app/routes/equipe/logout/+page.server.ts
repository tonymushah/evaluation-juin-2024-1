import { CLIENT_TOKEN_KEY } from '$lib/equipe/utils/tokenKey';
import { redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { route } from '$lib/ROUTES';

export const load: PageServerLoad = async ({ cookies }) => {
	cookies.delete(CLIENT_TOKEN_KEY, { path: '/equipe' });
	return redirect(300, route('/equipe/login'));
};
