import { graphql } from '$lib/equipe/gql';

export const query = graphql(`
	query currentEtape($etape: Int!) {
		etape {
			unique(rang: $etape) {
				depart
				finished
				nom
				rang
				longueur
			}
		}
	}
`);
