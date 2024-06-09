import { BACKEND_URL } from '$env/static/private';
import { CLIENT_TOKEN_KEY } from '$lib/equipe/utils/tokenKey';
import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { route } from '$lib/ROUTES';

export const ssr = false;

export const load: LayoutServerLoad = async function ({ cookies }) {
	const token = cookies.get(CLIENT_TOKEN_KEY);
	if (token) {
		return {
			client_url: `${BACKEND_URL}/equipe`,
			token
		};
	} else {
		redirect(300, route('/equipe/login'));
	}
};
