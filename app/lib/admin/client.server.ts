import { BACKEND_URL } from '$env/static/private';
import { Client, cacheExchange, fetchExchange, ssrExchange } from '@urql/svelte';

const isServerSide = typeof window === 'undefined';

const adminServerClient = new Client({
	url: `${BACKEND_URL}/admin`,
	exchanges: [fetchExchange]
});

export default adminServerClient;
