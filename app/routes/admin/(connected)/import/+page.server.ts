import adminServerClient from '$lib/admin/client.server';
import { graphql } from '$lib/admin/gql';
import { error, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { route } from '$lib/ROUTES';

export const ssr = false;

const pointsMutations = graphql(`
	mutation importPoints($file: Upload!) {
		import {
			points(file: $file) {
				rang
				valeur
			}
		}
	}
`);

const etapesMutations = graphql(`
	mutation importEtapes($file: Upload!) {
		import {
			etapes(file: $file) {
				rang
				depart
				finished
				nbCoureurParEquipe
			}
		}
	}
`);

const resultatsMutations = graphql(`
	mutation importResultats($file: Upload!) {
		import {
			resultats(file: $file) {
				idTempsCoureur
				temps
				points
				equipeCoureur
				etape
			}
		}
	}
`);

export const actions = {
	points: async ({ request }) => {
		const data = await request.formData();
		const file = data.get('file');
		if (file instanceof File) {
			const result = await adminServerClient
				.mutation(pointsMutations, {
					file
				})
				.toPromise();
			console.log(result.data);
			if (result.error?.message != undefined) {
				return error(500, { message: result.error?.message });
			}
			redirect(300, route('/admin/import'));
		} else {
			console.log('not a file');
			return error(400, { message: 'invalid input' });
		}
	},
	etapes: async ({ request }) => {
		const data = await request.formData();
		const file = data.get('file');
		if (file instanceof File) {
			const result = await adminServerClient
				.mutation(etapesMutations, {
					file
				})
				.toPromise();
			console.log(result.data);
			if (result.error?.message != undefined) {
				return error(500, { message: result.error?.message });
			}
			redirect(300, route('/admin/import'));
		} else {
			return error(400, { message: 'invalid input' });
		}
	},
	resultats: async ({ request }) => {
		const data = await request.formData();
		const file = data.get('file');
		console.log(data);
		if (file instanceof File) {
			const result = await adminServerClient
				.mutation(resultatsMutations, {
					file
				})
				.toPromise();
			console.log(result.data);
			console.log(result.error);
			if (result.error?.message != undefined) {
				return error(500, { message: result.error?.message });
			}
			redirect(300, route('/admin/import'));
		} else {
			console.log('not a file');
			return error(400, { message: 'invalid input' });
		}
	}
} satisfies Actions;
