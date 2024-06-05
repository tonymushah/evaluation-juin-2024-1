import { get_paginated_init_data, type PaginatedData } from '$lib';
import { graphql } from '$lib/global/gql';
import type { CoueurPoint, GraphQlOrdering } from '$lib/global/gql/graphql';
import { readonly, type Readable } from 'svelte/store';

const query = graphql(`
	query classementCategorie($ordre: GraphQLOrdering, $pagination: OffsetLimit, $categorie: UUID!) {
		classements {
			parCategorie(ordre: $ordre, pagination: $pagination, id: $categorie) {
				data {
					coureur
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

type ClassementGlobal = PaginatedData<CoueurPoint>;

export default function getClassement(categorie: string): ClassementGlobal {
	const { client, data, isLoading, hasNext, error } = get_paginated_init_data<CoueurPoint>();
	let offset = 0;
	let limit = 10;
	async function next() {
		isLoading.set(true);
		const res = await client
			.query(query, {
				pagination: { offset, limit },
				categorie
			})
			.toPromise()
			.finally(() => isLoading.set(false));
		if (res.data) {
			const resu = res.data.classements.parCategorie;
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
	}
	const obs = new IntersectionObserver(next);
	return {
		data: readonly(data),
		isLoading: readonly(isLoading),
		hasNext: readonly(hasNext),
		error: readonly(error),
		next,
		reset() {
			data.set([]);
			offset = 0;
			next();
		},
		obs
	};
}
