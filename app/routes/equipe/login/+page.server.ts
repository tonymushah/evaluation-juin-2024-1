import { graphql } from '$lib/equipe/gql';
import { CLIENT_TOKEN_KEY } from '$lib/equipe/utils/tokenKey';
import { error, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { route } from '$lib/ROUTES';
import equipeServerClient from '$lib/equipe/client.server';

export const ssr = true;

export const load: PageServerLoad = async function ({ cookies }) {
	if (cookies.get(CLIENT_TOKEN_KEY)) {
		redirect(300, route('/equipe'));
	}
};

const loginMutation = graphql(`
	mutation equipeLogin($user: String!, $password: String!) {
		login(username: $user, password: $password)
	}
`);

export const actions = {
	default: async ({ request, cookies }) => {
		const form = await request.formData();
		const user = form.get('pseudo');
		const password = form.get('password');
		if (typeof user != 'string' || typeof password != 'string') {
			error(400, {
				message: 'invalid input'
			});
		}
		const res = await equipeServerClient
			.mutation(loginMutation, {
				user,
				password
			})
			.toPromise();
		if (res.data) {
			cookies.set(CLIENT_TOKEN_KEY, res.data.login, {
				path: '/'
			});
			redirect(300, route('/equipe'));
		} else if (res.error) {
			error(500, res.error);
		}
		return;
	}
} satisfies Actions;
