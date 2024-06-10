import type { PageServerLoad } from './$types';

export const ssr = false;

export const load: PageServerLoad = async ({ cookies, params }) => {
	return {
		rang: Number(params.id)
	};
};
