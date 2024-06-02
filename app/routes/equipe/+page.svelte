<script lang="ts">
	import { brand } from '$lib';
	import EtapeCardList from '$lib/equipe/componnents/etape/EtapeCardList.svelte';
	import { v4 } from 'uuid';
</script>

<svelte:head>
	<title>Votre equipe | {brand}</title>
</svelte:head>

<h2 class="text-2xl font-semibold underline">Etapes</h2>

<EtapeCardList
	etapes={[
		{
			etat: 'Null',
			id: v4(),
			nom: 'Nom equipe',
			points: 5,
			joueurs: [
				{
					nom: 'tony',
					points: 5
				},
				{
					nom: 'tony',
					points: 5
				},
				{
					nom: 'tony',
					points: 5
				},
				{
					nom: 'tony',
					points: 5
				},
				{
					nom: 'tony',
					points: 5
				}
			]
		}
	]}
/>
