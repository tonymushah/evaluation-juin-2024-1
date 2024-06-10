<script lang="ts">
	import { brand } from '$lib';
	import EtapeCardList from '$lib/equipe/componnents/etape/EtapeCardList.svelte';
	import type { PageServerData } from './$types';
</script>

<svelte:head>
	<title>Votre equipe | {brand}</title>
</svelte:head>

<h2 class="text-2xl font-semibold">Etapes</h2>

<EtapeCardList />
