<script lang="ts">
	import CoueurSelector from '$lib/equipe/componnents/etape/page/CoueurSelector.svelte';
	import { onMount, type ComponentProps } from 'svelte';
	import { get, writable } from 'svelte/store';
	import { getSelectedJoueurStore } from './+layout.svelte';
	import getClassement from '$lib/equipe/componnents/etape/listCoureur.query';
	const { data, hasNext, isLoading, reset, obs } = getClassement();
	let toObs: HTMLDivElement | undefined = undefined;
	$: {
		if (toObs) obs.observe(toObs);
	}
	onMount(async () => {
		await reset();
	});

	const selected = getSelectedJoueurStore();
</script>

<div class="flex flex-wrap gap-2 mt-3">
	{#each $data as coueur}
		<CoueurSelector
			nom={coueur.nom}
			isSelected={selected.listenToCoeur(coueur.id)}
			on:click={() => {
				if (!get(selected.listenToCoeur(coueur.id))) {
					selected.select(coueur.id);
				} else {
					selected.unselect(coueur.id);
				}
			}}
		/>
	{/each}
</div>

{#if !$isLoading && $hasNext}
	<div />
{/if}
