import { error, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { route } from '$lib/ROUTES';
import { graphql } from '$lib/admin/gql';
import adminServerClient from '$lib/admin/client.server';

const resetMutation = graphql(`
	mutation resetDB {
		resetDb
	}
`);

export const load: PageServerLoad = async () => {
	const res = await adminServerClient.mutation(resetMutation, {}).toPromise();
	if (res.error) {
		error(500, res.error.message);
	}
	redirect(300, route('/admin'));
};
