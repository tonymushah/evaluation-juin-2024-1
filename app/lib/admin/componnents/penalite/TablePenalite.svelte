<script lang="ts">
	import Row from './TablePenaliteRow.svelte';
	import TablePenaliteHead from './TablePenaliteHead.svelte';
	import { Button, Table, TableBody } from 'flowbite-svelte';
	import getClassement from './query';
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';

	const { data, hasNext, isLoading, reset, next } = getClassement();
	onMount(async () => {
		await reset();
	});
</script>

<Button disabled={$isLoading} on:click={reset}>Reset</Button>

{#if $isLoading}
	<h3 transition:slide>Loading...</h3>
{/if}

{#if $data.length > 0}
	<Table color="red">
		<TablePenaliteHead />
		<TableBody tableBodyClass="divide-y">
			{#each $data as class_}
				<Row etape={class_.etape} equipe={class_.equipe} temps={class_.temps}>
					<form slot="fourth-col" method="post" action="?/delete">
						<input hidden name="penalite" value={class_.id} />
						<Button type="submit">Delete</Button>
					</form>
				</Row>
			{/each}
		</TableBody>
	</Table>
{:else}
	<div class="flex w-full justify-center items-center">
		<h2>Aucune penalite en cours...</h2>
	</div>
{/if}

{#if $hasNext && !$isLoading}
	<div class="flex w-full items-center justify-center">
		<Button
			disabled={$isLoading}
			on:click={async () => {
				await next();
			}}
		>
			Next
		</Button>
	</div>
{/if}
