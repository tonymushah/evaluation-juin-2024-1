import { graphql } from '$lib/equipe/gql';
import { getContextClient, queryStore } from '@urql/svelte';

const query = graphql(`
	query etapeCoureurs($etape: Int!) {
		etape {
			joueurs(etape: $etape) {
				coureur
				nomCoureur
				points
			}
		}
	}
`);

export default function getEtapeCoueur(etape: number) {
	const client = getContextClient();
	return queryStore({
		client,
		query,
		variables: {
			etape
		}
	});
}
