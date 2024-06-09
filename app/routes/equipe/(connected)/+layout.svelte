<script lang="ts">
	import { Client, fetchExchange, setContextClient } from '@urql/svelte';
	import type { LayoutServerData } from './$types';
	import TopNavBar from '$lib/equipe/componnents/TopNavBar.svelte';

	export let data: LayoutServerData;
	const headers = new Headers();
	headers.append('authorization', data.token);
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

<TopNavBar />

<div class="content ml-5 mr-5 mt-5">
	<slot />
</div>
