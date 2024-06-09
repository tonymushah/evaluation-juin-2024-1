import equipeServerClient from '$lib/equipe/client.server';
import { graphql } from '$lib/equipe/gql';
import currentEquipeQuery from '$lib/equipe/utils/queries/currentEquipe';
import { CLIENT_TOKEN_KEY } from '$lib/equipe/utils/tokenKey';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies }) => {
	let token = cookies.get(CLIENT_TOKEN_KEY);
	let headers = new Headers();
	headers.append('authorization', token ?? '');
	const data = await equipeServerClient
		.query(
			currentEquipeQuery,
			{},
			{
				fetchOptions: {
					headers
				}
			}
		)
		.toPromise();
	if (data.data) {
		return {
			current: data.data.current
		};
	} else if (data.error) {
		return error(500, data.error);
	} else {
		return error(500, {
			message: 'Unexepected error'
		});
	}
};
