import { BACKEND_URL } from '$env/static/private';
import { Client, cacheExchange, fetchExchange, ssrExchange } from '@urql/svelte';

const isServerSide = typeof window === 'undefined';

const equipeServerClient = new Client({
	url: `${BACKEND_URL}/equipe`,
	exchanges: [fetchExchange]
});

export default equipeServerClient;
