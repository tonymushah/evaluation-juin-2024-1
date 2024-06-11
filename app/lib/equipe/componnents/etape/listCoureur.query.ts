import { get_paginated_init_data, type PaginatedData } from '$lib';
import { graphql } from '$lib/equipe/gql';
import { get, readonly } from 'svelte/store';

const query = graphql(`
	query listEquipeCoureur($page: OffsetLimit!) {
		listCoureur(pagination: $page) {
			data {
				dtn
				nomCoureur
				genre
				points
				coureur
			}
			limit
			offset
			total
		}
	}
`);

type CoureurItem = {
	id: number;
	nom: string;
};

type ClassementGlobal = PaginatedData<CoureurItem>;

export default function getClassement(): ClassementGlobal {
	const { client, data, isLoading, hasNext, error } = get_paginated_init_data<CoureurItem>();
	let offset = 0;
	let limit = 10;
	async function next() {
		if (!get(isLoading) && get(hasNext)) {
			isLoading.set(true);
			const res = await client
				.query(query, {
					page: { offset, limit }
				})
				.toPromise();
			if (res.data) {
				const resu = res.data.listCoureur;
				data.update((a) => {
					a.push(
						...resu.data.map((e) => ({
							id: e.coureur,
							nom: e.nomCoureur
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
		if (!get(isLoading) || get(hasNext)) next();
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
