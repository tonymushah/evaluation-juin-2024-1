import { formatSecond, get_paginated_init_data, type PaginatedData } from '$lib';
import { graphql } from '$lib/admin/gql';
import { readonly } from 'svelte/store';

const query = graphql(`
	query penalites($page: OffsetLimit!) {
		penalite {
			list(pagination: $page) {
				data {
					idPenalite
					etapeData {
						nom
					}
					equipeData {
						nom
					}
					valeur
				}
				offset
				total
				limit
			}
		}
	}
`);

type PenaliteData = {
	id: string;
	temps: string;
	etape: string;
	equipe: string;
};

type ClassementGlobal = PaginatedData<PenaliteData>;

export default function getClassement(): ClassementGlobal {
	const { client, data, isLoading, hasNext, error } = get_paginated_init_data<PenaliteData>();
	let offset = 0;
	let limit = 10;
	async function next() {
		isLoading.set(true);
		const res = await client
			.query(query, {
				page: { offset, limit }
			})
			.toPromise();
		if (res.data) {
			const resu = res.data.penalite.list;
			data.update((a) => {
				a.push(
					...resu.data.map((e) => ({
						id: e.idPenalite,
						temps: formatSecond(e.valeur),
						etape: e.etapeData.nom,
						equipe: e.equipeData.nom
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
