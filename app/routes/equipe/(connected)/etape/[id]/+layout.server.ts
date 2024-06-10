import equipeServerClient from '$lib/equipe/client.server';
import { query as currentEtapeQuery } from '$lib/equipe/componnents/etape/currentEtape.query';
import { getClientHeaders } from '$lib/equipe/utils/getToken.server';
import { error } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const ssr = true;

export const load: LayoutServerLoad = async ({ cookies, params }) => {
	const headers = getClientHeaders(cookies);

	const res = await equipeServerClient
		.query(
			currentEtapeQuery,
			{
				etape: Number(params.id)
			},
			{
				fetchOptions: {
					headers
				}
			}
		)
		.toPromise();
	if (res.data) {
		const data = res.data.etape.unique;
		return {
			nom_etape: data.nom,
			depart: data.depart,
			rang: data.rang,
			kilometrage: Number(data.longueur)
		};
	} else if (res.error) {
		error(500, res.error);
	} else {
		error(404, 'Not found');
	}
};
