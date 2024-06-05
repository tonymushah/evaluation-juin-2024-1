import { graphql } from '$lib/admin/gql/gql.js';
import { error } from '@sveltejs/kit';
import type { Actions } from './$types';
import adminServerClient from '$lib/admin/client.server';
import { _temps } from '$lib';
import { v4 } from 'uuid';

export const ssr = false;

const addMutation = graphql(`
	mutation addPenaliteAction($input: PenaliteInput!) {
		penalite {
			upsert(input: $input) {
				etape
				equipe
				valeur
			}
		}
	}
`);

const removeMutation = graphql(`
	mutation removePenaliteAction($input: UUID!) {
		penalite {
			remove(id: $input) {
				etape
				equipe
				valeur
			}
		}
	}
`);

export const actions = {
	add: async ({ request }) => {
		const data = await request.formData();
		const equipe = data.get('equipe');
		const etape = data.get('etape');
		const temps = _temps(data.get('temps'));
		if (typeof equipe != 'string' || typeof equipe != 'string' || typeof temps != 'number') {
			return error(400, {
				message: 'Invalid Input'
			});
		} else {
			const res = await adminServerClient
				.mutation(addMutation, {
					input: {
						equipe,
						etape: Number(etape),
						valeur: temps,
						idPenalite: v4()
					}
				})
				.toPromise();
			if (res.error) {
				return error(500, res.error);
			}
		}
	},
	delete: async ({ request }) => {
		const data = await request.formData();
		const penalite = data.get('penalite');
		if (typeof penalite != 'string') {
			return error(500, {
				message: 'Invalid input'
			});
		} else {
			const res = await adminServerClient
				.mutation(removeMutation, {
					input: penalite
				})
				.toPromise();
			if (res.error) {
				return error(500, res.error);
			}
		}
	}
} satisfies Actions;
