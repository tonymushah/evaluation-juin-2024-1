<script lang="ts">
	import Row from './TablePenaliteRow.svelte';
	import TablePenaliteHead from './TablePenaliteHead.svelte';
	import { Button, Table, TableBody } from 'flowbite-svelte';
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

<Table color="blue">
	<TablePenaliteHead />
	<TableBody tableBodyClass="divide-y">
		{#each $data as class_}
			<Row etape={class_.etape} equipe={class_.equipe} temps={class_.temps} />
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
