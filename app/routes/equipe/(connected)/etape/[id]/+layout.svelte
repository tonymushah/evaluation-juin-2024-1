<script lang="ts" context="module">
	import { writable, type Writable } from 'svelte/store';
	import { getContext } from 'svelte';

	const pointsContextKey = 'ETAPE-SUB-POINT-KEY';
	export function getPointsContextKey(): Writable<number> {
		const context = getContext<Writable<number> | undefined>(pointsContextKey);
		if (context) {
			return context;
		} else {
			throw new Error(`The ${pointsContextKey} should be defined`);
		}
	}
</script>

<script lang="ts">
	import type { LayoutServerData } from './$types';
	import { setContext } from 'svelte';
	export let data: LayoutServerData;
	const points = setContext(pointsContextKey, writable(0));
</script>

<div class="grid grid-cols-2">
	<h2 class="text-xl font-semibold underline">{data.nom_etape}</h2>
	<p class="text-lg">
		Points: {$points}
		{#if $points > 1}
			pts
		{:else}
			pt
		{/if}
	</p>
</div>
