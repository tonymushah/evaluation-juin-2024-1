<script lang="ts">
	import TopNavbar from '$lib/admin/componnents/TopNavbar.svelte';
	import { Client, cacheExchange, fetchExchange, setContextClient } from '@urql/svelte';
	import type { LayoutServerData } from './$types';
	export let data: LayoutServerData;
	setContextClient(
		new Client({
			url: `${data.BACKEND_URL}/admin`,
			exchanges: [fetchExchange, cacheExchange]
		})
	);
</script>

<TopNavbar />
<div class="border px-5 py-2 rounded-lg bg-white dark:bg-gray-600">
	<slot />
</div>
