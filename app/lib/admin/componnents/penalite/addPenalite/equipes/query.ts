import { get_paginated_init_data, type PaginatedData } from '$lib';
import { graphql } from '$lib/admin/gql';
import { get, readonly } from 'svelte/store';

const query = graphql(`
	query selectEquipe($page: OffsetLimit!) {
		equipe {
			list(pagination: $page) {
				data {
					nom
					idEquipe
				}
				limit
				total
				offset
			}
		}
	}
`);

type EquipeData = {
	id: string;
	nom: string;
};

type ClassementGlobal = PaginatedData<EquipeData>;

export default function getClassement(): ClassementGlobal {
	const { client, data, isLoading, hasNext, error } = get_paginated_init_data<EquipeData>();
	let offset = 0;
	let limit = 10;
	async function next() {
		if (!get(isLoading)) {
			isLoading.set(true);
			const res = await client
				.query(query, {
					page: { offset, limit }
				})
				.toPromise();
			if (res.data) {
				const resu = res.data.equipe.list;
				data.update((a) => {
					a.push(
						...resu.data.map((e) => ({
							id: e.idEquipe,
							nom: e.nom
						}))
					);
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
			data.set([]);
			offset = 0;
			next();
		},
		obs
	};
}
