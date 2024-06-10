<script lang="ts">
	import { onMount, type ComponentProps } from 'svelte';
	import EtapeCard from './EtapeCard.svelte';
	import getClassement from './list.query';
	import { derived } from 'svelte/store';

	const { data, hasNext, isLoading, reset, next, obs } = getClassement();
	let obss: HTMLDivElement | undefined = undefined;
	$: {
		if (obss) obs.observe(obss);
	}
	onMount(async () => {
		await reset();
	});
	const etapes = derived(data, (d) =>
		d.map<ComponentProps<EtapeCard>>((i) => ({
			id: i.id,
			nom: i.nom,
			etat: i.etat ?? 'Not Definie'
		}))
	);
</script>

<div class="flex flex-col mt-4">
	{#each $etapes as etape}
		<EtapeCard {...etape} />
	{:else}
		<h3 class="text-md">Rien a voir ici</h3>
	{/each}
</div>

{#if !$isLoading && $hasNext}
	<div bind:this={obss} />
{/if}
