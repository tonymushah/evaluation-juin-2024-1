<script lang="ts">
	import TableEtapeRow from '$lib/admin/componnents/etape/TableEtapeRow.svelte';
	import { Button, Table, TableBody, TableHead, TableHeadCell } from 'flowbite-svelte';
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

{#if $data.length > 0}
	<Table color="blue">
		<TableHead>
			<TableHeadCell>Rang</TableHeadCell>
			<TableHeadCell>Nom etape</TableHeadCell>
			<TableHeadCell>Nb Coureurs</TableHeadCell>
			<TableHeadCell>Kilometrage</TableHeadCell>
			<TableHeadCell>Date Debut</TableHeadCell>
			<TableHeadCell>Date Fin</TableHeadCell>
		</TableHead>
		<TableBody tableBodyClass="divide-y">
			{#each $data as class_}
				<TableEtapeRow
					rang={class_.rang}
					nom={class_.nom}
					kilometrage={class_.longueur}
					dateDebut={class_.depart}
					dateFin={class_.finished}
					nbCoureur={class_.nbCoureur}
				/>
			{/each}
		</TableBody>
	</Table>
{:else}
	<div class="flex w-full justify-center items-center">
		<h2 class="text-xl">Rien a voir??</h2>
	</div>
{/if}

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
