import { get_paginated_init_data, type PaginatedData } from '$lib';
import { graphql } from '$lib/equipe/gql';
import { readonly, get } from 'svelte/store';

export const query = graphql(`
	query listEtapes($page: OffsetLimit!) {
		etape {
			list(pagination: $page) {
				data {
					rang
					longueur
					nom
					depart
					finished
				}
				limit
				offset
				total
			}
		}
	}
`);

type EtapeListItem = {
	id: number;
	nom: string;
	etat?: string;
	depart?: Date;
};

type ClassementGlobal = PaginatedData<EtapeListItem>;

export default function getClassement(): ClassementGlobal {
	const { client, data, isLoading, hasNext, error } = get_paginated_init_data<EtapeListItem>();
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
				const resu = res.data.etape.list;
				data.update((a) => {
					a.push(
						...resu.data.map((e) => ({
							id: e.rang,
							nom: e.nom,
							depart: new Date(e.depart),
							etat: e.finished == undefined ? 'Inconnue' : 'Finie'
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
