<script lang="ts">
	import { TabItem, Table, Tabs } from 'flowbite-svelte';
	import Generale, {
		type ClassementItem as CGeneItem
	} from '$lib/global/componnents/classement/general/TableClassmmentGenerale.svelte';
	import Equipes, {
		type ClassementItem as CEqupeItem
	} from '$lib/global/componnents/classement/equipe/TableClassmmentEquipe.svelte';
	import { readable } from 'svelte/store';
	import { brand } from '$lib';
	import type { PageServerData } from './$types';
	import { Client, cacheExchange, fetchExchange, setContextClient } from '@urql/svelte';

	export let data: PageServerData;
	setContextClient(
		new Client({
			url: data.client_url,
			exchanges: [fetchExchange, cacheExchange]
		})
	);
</script>

<h2 class="text-2xl font-bold underline">Classements global</h2>

<svelte:head>
	<title>Classement global | {brand}</title>
</svelte:head>

<Tabs class="mt-2">
	<TabItem open title="Par Coureur">
		<Generale />
	</TabItem>
	<TabItem title="Par Equipe">
		<Equipes />
	</TabItem>
</Tabs>
