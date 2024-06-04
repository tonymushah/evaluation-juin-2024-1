<script lang="ts">
	import { Button, Table, TableBody } from 'flowbite-svelte';
	import TClassHead from './TClassHead.svelte';
	import TClassRow from './TClassRow.svelte';
	import getClassement from './query';
	import { onMount } from 'svelte';

	const { data, hasNext, isLoading, reset, next } = getClassement();
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
			<TClassRow equipe={class_.nom} points={class_.points} />
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
