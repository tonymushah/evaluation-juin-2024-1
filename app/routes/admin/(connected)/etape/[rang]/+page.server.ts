import adminServerClient from '$lib/admin/client.server';
import { graphql } from '$lib/admin/gql';
import { error, redirect } from '@sveltejs/kit';
import type { Action, Actions, PageServerLoad } from './$types';
import { tempsRegex, toSecond, _temps as _t } from '$lib';
import { route } from '$lib/ROUTES';

export const ssr = true;

const pageQuery = graphql(`
	query getAdminEtape($rang: Int!, $courPage: OffsetLimit!) {
		etape {
			unique(rang: $rang) {
				depart
				finished
				rang
				nom
				nbCoureurParEquipe
				longueur
				coureur(pagination: $courPage) {
					data {
						temps
						points
						coureur {
							numeroDosard
							nom
						}
						equipeCoureur
						equipe {
							nom
						}
					}
					limit
					total
					offset
				}
			}
		}
	}
`);

export const load: PageServerLoad = async ({ params, url }) => {
	const o = url.searchParams.get('offset');
	const l = url.searchParams.get('limit');
	const offset = o != null ? Number(o) : 0;
	const limit = l != null ? Number(l) : 10;
	const res = await adminServerClient
		.query(pageQuery, {
			rang: Number(params.rang),
			courPage: {
				offset,
				limit
			}
		})
		.toPromise();
	if (res.data) {
		const data = res.data.etape.unique;
		return {
			nom_etape: data.nom,
			depart: data.depart,
			rang: data.rang,
			kilometrage: Number(data.longueur),
			coureurs: {
				list: data.coureur.data,
				offset,
				limit,
				hasNext: data.coureur.offset + data.coureur.limit > data.coureur.limit,
				hasPrevious: data.coureur.offset - data.coureur.limit <= 0,
				total: data.coureur.total
			}
		};
	} else if (res.error) {
		error(500, res.error);
	} else {
		error(404, 'Not found');
	}
};

function _temps(data: FormData): number | undefined {
	const temps = data.get('temps');
	return _t(temps);
}

const add_arrivee_mutation = graphql(`
	mutation addTimeToJoueur($etape: Int!, $dosard: Int!, $temps: TempsCoureur!) {
		etape(id: $etape) {
			addTime(dosard: $dosard, temps: $temps) {
				temps
				etape
				idTempsCoureur
				points
			}
		}
	}
`);

export const actions = {
	add_arrivee: async ({ request, params }) => {
		const data = await request.formData();
		const temps = _temps(data);
		if (temps) {
			error(400, {
				message: 'Invalid temps input'
			});
		}
		const dosard = data.get('dosard');
		if (typeof dosard != 'string') {
			error(400, {
				message: 'Invalid dosard input'
			});
		}
		const num_dosard = Number(dosard);
		if (!isFinite(num_dosard)) {
			error(400, {
				message: 'Invalid dosard input'
			});
		}
		const result = await adminServerClient
			.mutation(add_arrivee_mutation, {
				etape: Number(params.rang),
				dosard: num_dosard,
				temps
			})
			.toPromise();
		if (result.data) {
			redirect(300, route('/admin/etape/[rang]', { rang: params.rang }));
		} else if (result.error) {
			error(500, {
				message: result.error.message
			});
		} else {
			redirect(300, route('/admin/etape/[rang]', { rang: params.rang }));
		}
	}
} satisfies Actions;
