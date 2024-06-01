import { BACKEND_URL } from '$env/static/private';
import { Client, cacheExchange, fetchExchange, ssrExchange } from '@urql/svelte';

const isServerSide = typeof window === 'undefined';

// The `ssrExchange` must be initialized with `isClient` and `initialState`
const ssr = ssrExchange({
	isClient: !isServerSide,
	initialState: !isServerSide ? window.__URQL_DATA__ : undefined
});

const equipeServerClient = new Client({
	url: `${BACKEND_URL}/equipe`,
	exchanges: [
		cacheExchange,
		ssr, // Add `ssr` in front of the `fetchExchange`
		fetchExchange
	]
});

export default equipeServerClient;
