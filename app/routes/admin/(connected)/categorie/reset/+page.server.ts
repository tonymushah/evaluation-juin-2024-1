import adminServerClient from '$lib/admin/client.server';
import { graphql } from '$lib/admin/gql';
import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { route } from '$lib/ROUTES';

const mutation = graphql(`
	mutation resetCat {
		truncateCategories
	}
`);

export const load: PageServerLoad = async () => {
	const res = await adminServerClient.mutation(mutation, {}).toPromise();
	if (res.error) {
		return error(500, res.error);
	}
	return redirect(300, route('/admin'));
};
