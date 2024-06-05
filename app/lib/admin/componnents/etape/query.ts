import { get_paginated_init_data, type PaginatedData } from '$lib';
import { graphql } from '$lib/admin/gql';
import { readonly } from 'svelte/store';

const query = graphql(`
	query listAdminEtape($pagination: OffsetLimit!) {
		etape {
			list(pagination: $pagination) {
				data {
					depart
					finished
					rang
					depart
					nom
					longueur
					coureur(pagination: { limit: 1, offset: 0 }) {
						total
					}
				}
				offset
				limit
				total
			}
		}
	}
`);

type AdminEtape = {
	rang: number;
	longueur: number;
	nom: string;
	nbCoureur: number;
	depart: Date;
	finished?: Date;
};

type ClassementGlobal = PaginatedData<AdminEtape>;

export default function getClassement(): ClassementGlobal {
	const { client, data, isLoading, hasNext, error } = get_paginated_init_data<AdminEtape>();
	let offset = 0;
	let limit = 10;
	async function next() {
		isLoading.set(true);
		const res = await client
			.query(query, {
				pagination: { offset, limit }
			})
			.toPromise();
		if (res.data) {
			const resu = res.data.etape.list;
			data.update((a) => {
				a.push(
					...resu.data.map((e) => ({
						rang: e.rang,
						depart: new Date(e.depart),
						finished: e.finished != undefined ? new Date(e.finished) : undefined,
						nbCoureur: e.coureur.total,
						nom: e.nom,
						longueur: Number(e.longueur)
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
