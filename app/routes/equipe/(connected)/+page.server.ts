import equipeServerClient from '$lib/equipe/client.server';
import { graphql } from '$lib/equipe/gql';
import { CLIENT_TOKEN_KEY } from '$lib/equipe/utils/tokenKey';
import type { PageServerLoad } from './$types';

const query = graphql(`
	query me {
		current {
			pseudo
			nom
		}
	}
`);

export const load: PageServerLoad = async ({ cookies }) => {
	let token = cookies.get(CLIENT_TOKEN_KEY);
	let headers = new Headers();
	headers.append('authorization', token ?? '');
	const data = await equipeServerClient
		.query(
			query,
			{},
			{
				fetchOptions: {
					headers
				}
			}
		)
		.toPromise();
};
