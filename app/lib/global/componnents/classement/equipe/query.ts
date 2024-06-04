import { get_paginated_init_data, type PaginatedData } from '$lib';
import { graphql } from '$lib/global/gql';
import type { EquipePoint, GraphQlOrdering } from '$lib/global/gql/graphql';
import { readonly, type Readable } from 'svelte/store';

const query = graphql(`
	query classementEquipeGenerale($ordre: GraphQLOrdering = DESCENDING, $pagination: OffsetLimit) {
		classements {
			parEquipe(ordre: $ordre, pagination: $pagination) {
				data {
					equipe
					nom
					temps
					points
				}
				limit
				offset
				total
			}
		}
	}
`);

type ClassementGlobal = PaginatedData<EquipePoint>;

export default function getClassement(ordre?: Readable<GraphQlOrdering>): ClassementGlobal {
	const { client, data, isLoading, hasNext, error } = get_paginated_init_data<EquipePoint>();
	let offset = 0;
	let limit = 10;
	async function next() {
		isLoading.set(true);
		const res = await client
			.query(query, {
				pagination: { offset, limit }
			})
			.toPromise()
			.catch((e) => {
				isLoading.set(false);
				return e;
			});
		if (res.data) {
			const resu = res.data.classements.parEquipe;
			data.update((a) => {
				a.push(...resu.data);
				return a;
			});
			const next_offset = resu.offset + resu.limit;
			if (next_offset > resu.total) {
				hasNext.set(false);
			} else {
				offset = next_offset;
			}
		} else if (res.error) {
			error.set(res.error);
		}
		isLoading.set(false);
	}
	const obs = new IntersectionObserver(() => {
		return isLoading.subscribe((is) => {
			if (!is) {
				next();
			}
		});
	});
	return {
		data: readonly(data),
		isLoading: readonly(isLoading),
		hasNext: readonly(hasNext),
		error: readonly(error),
		next,
		reset() {
			offset = 0;
			next();
		},
		obs
	};
}
