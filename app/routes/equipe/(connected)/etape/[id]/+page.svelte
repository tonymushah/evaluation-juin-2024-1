<script lang="ts">
	import CoueurSelector from '$lib/equipe/componnents/etape/page/CoueurSelector.svelte';
	import type { ComponentProps } from 'svelte';
	import { writable } from 'svelte/store';

	const testData = writable<ComponentProps<CoueurSelector>[]>([
		{
			nom: 'Tony',
			isSelected: false
		},
		{
			nom: 'Akari',
			isSelected: false
		},
		{
			nom: 'Tomfy',
			isSelected: false
		}
	]);
</script>

<div class="flex flex-wrap gap-2 mt-3">
	{#each $testData as coueur}
		<CoueurSelector
			{...coueur}
			on:click={() => {
				testData.update((c) => {
					const selected = c.find((c) => c.nom == coueur.nom);
					if (selected) {
						selected.isSelected = !selected.isSelected;
					}
					return c;
				});
			}}
		/>
	{/each}
</div>
