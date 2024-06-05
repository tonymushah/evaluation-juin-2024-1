<script lang="ts">
	import { Client, fetchExchange, setContextClient } from '@urql/svelte';
	import type { LayoutServerData } from './$types';
	import getToken from '$lib/equipe/utils/getToken';

	export let data: LayoutServerData;
	const token = getToken();
	const headers = new Headers();
	headers.append('authorization', token!);
	setContextClient(
		new Client({
			url: data.client_url,
			exchanges: [fetchExchange],
			fetchOptions: {
				headers
			}
		})
	);
</script>

<div class="content ml-5 mr-5 mt-5">
	<slot />
</div>
