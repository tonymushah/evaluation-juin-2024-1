import { BACKEND_URL } from '$env/static/private';
import type { PageServerLoad } from './$types';

export const ssr = false;

export const load: PageServerLoad = async () => {
	return {
		client_url: BACKEND_URL
	};
};
