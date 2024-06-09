import { graphql } from '$lib/equipe/gql';
import { getContextClient, queryStore } from '@urql/svelte';

export const currentEquipeQuery = graphql(`
	query currentEquipe {
		current {
			pseudo
			nom
			idEquipe
		}
	}
`);

export default function getCurrent() {
	const client = getContextClient();
	return queryStore({
		client,
		query: currentEquipeQuery,
		variables: {}
	});
}
