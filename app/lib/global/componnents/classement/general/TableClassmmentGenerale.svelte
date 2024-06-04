<script lang="ts" context="module">
	export type ClassementItem = {
		coureur: string;
		points: number;
	};
</script>

<script lang="ts">
	import { Button, Table, TableBody } from 'flowbite-svelte';
	import TClassHead from './TClassHead.svelte';
	import TClassRow from './TClassRow.svelte';
	import { readable, type Readable } from 'svelte/store';
	import getClassement from './query';

	const { data, hasNext, isLoading, obs, reset } = getClassement();
	let toObserve: HTMLElement | undefined = undefined;
	$: {
		if (toObserve) {
			obs.observe(toObserve);
		}
	}
</script>

{#if $isLoading}
	<h3>Loading...</h3>
{/if}
<Button disabled={$isLoading} on:click={reset}>Reset</Button>
<Table>
	<TClassHead />
	<TableBody tableBodyClass="divide-y divide-x">
		{#each $data as class_}
			<TClassRow coureur={class_.coureur.toFixed()} points={class_.points} temps={class_.temps} />
		{/each}
		{#if $hasNext && !$isLoading}
			<div bind:this={toObserve} />
		{/if}
	</TableBody>
</Table>
