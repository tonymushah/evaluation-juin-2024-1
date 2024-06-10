import { redirect, type Cookies } from '@sveltejs/kit';
import { CLIENT_TOKEN_KEY } from './tokenKey';
import { route } from '$lib/ROUTES';

export default function getToken(cookies: Cookies) {
	const token = cookies.get(CLIENT_TOKEN_KEY);
	if (token) {
		return token;
	} else {
		redirect(300, route('/equipe/login'));
	}
}

export function getClientHeaders(cookies: Cookies): Headers {
	let headers = new Headers();
	headers.set('authorization', getToken(cookies));
	return headers;
}
