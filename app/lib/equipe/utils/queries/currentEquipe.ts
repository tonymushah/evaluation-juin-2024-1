import { graphql } from '$lib/equipe/gql';

const currentEquipeQuery = graphql(`
	query me {
		current {
			pseudo
			nom
		}
	}
`);

export default currentEquipeQuery;
