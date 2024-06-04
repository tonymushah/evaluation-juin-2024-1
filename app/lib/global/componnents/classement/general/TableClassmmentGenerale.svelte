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
	import getClassement from './query';
	import { onMount } from 'svelte';

	const { data, hasNext, isLoading, obs, reset, next } = getClassement();
	onMount(async () => {
		await reset();
	});
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
	</TableBody>
</Table>
{#if $hasNext && !$isLoading}
	<div class="flex w-full items-center justify-center">
		<Button
			disabled={$isLoading}
			on:click={async () => {
				await next();
			}}>Next</Button
		>
	</div>
{/if}
