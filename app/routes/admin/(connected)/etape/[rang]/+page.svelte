<script lang="ts">
	import TableCoureurEntreeRow from '$lib/admin/componnents/etape/table/TableCoureurEntreeRow.svelte';
	import {
		Button,
		Input,
		Label,
		Table,
		TableBody,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';
	import type { PageServerData } from './$types';
	import { setContext } from 'svelte';
	import { brand } from '$lib';
	import { route } from '$lib/ROUTES';
	export let data: PageServerData;
	setContext('etape-rang', data.rang);
</script>

<svelte:head>
	<title>Etape {data.nom_etape} | {brand}</title>
</svelte:head>

<div class="flex flex-row gap-4 items-center mb-3">
	<h2 class="text-xl font-semibold">{data.nom_etape}</h2>
	<div class="grid grid-cols-2 gap-2">
		<p>Kilometrage: {data.kilometrage} km</p>
		<p>Debut: {new Date(data.depart).toDateString()}</p>
		<p>Nombre Coureur: {data.coureurs.total}</p>
	</div>
</div>

<div class="flex flex-row gap-4 items-center">
	<div class="flex flex-col gap-2">
		<Label>Equipe</Label>
		<Input placeholder="Nom de l'equipe" />
	</div>
	<div class="flex flex-col gap-2">
		<Label>Coueur</Label>
		<Input placeholder="Nom du coureur" />
	</div>
</div>

{#if data.coureurs.hasNext || data.coureurs.hasPrevious}
	<div class="flex p-4 gap-2">
		{#if data.coureurs.hasNext}
			<Button href={`?offset=${data.coureurs.offset - data.coureurs.limit}`}>Previous</Button>
		{/if}

		{#if data.coureurs.hasNext}
			<Button href={`?offset=${data.coureurs.offset + data.coureurs.limit}`}>Next</Button>
		{/if}
	</div>
{/if}

<div class="mt-2">
	<Table color="green">
		<TableHead>
			<TableHeadCell>Coureur</TableHeadCell>
			<TableHeadCell>Equipe</TableHeadCell>
			<TableHeadCell>Dosard</TableHeadCell>
			<TableHeadCell>Arrivee</TableHeadCell>
			<TableHeadCell>Points</TableHeadCell>
		</TableHead>
		<TableBody>
			{#each data.coureurs.list as coureur}
				<TableCoureurEntreeRow
					coureur={coureur.coureur.nom}
					equipe={coureur.equipe.nom}
					dosard={coureur.coureur.numeroDosard}
					arrivee={typeof coureur.temps == 'number' ? coureur.temps : undefined}
					points={typeof coureur.points == 'number' ? coureur.points : undefined}
				/>
			{/each}
		</TableBody>
	</Table>
</div>
