<script lang="ts">
	import TableCoureurEntreeRow from '$lib/admin/componnents/etape/table/TableCoureurEntreeRow.svelte';
	import { Input, Label, Table, TableBody, TableHead, TableHeadCell } from 'flowbite-svelte';
	import { writable } from 'svelte/store';
</script>

<div class="flex flex-row gap-4 items-center">
	<h2 class="text-xl font-semibold">%NOM_ETAPE%</h2>
	<div class="grid grid-cols-2">
		<p>Lieu: %LIEU%</p>
		<p>Kilometrage: %KILOMETRAGE% km</p>
		<p>Debut: %DATE_DEBUT%</p>
		<p>Etat: %Etat_etape%</p>
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
			<TableCoureurEntreeRow
				coureur="Tony"
				equipe="Walawala"
				dosard={1}
				arrivee={writable(undefined)}
			/>
			<TableCoureurEntreeRow
				coureur="Tomefy"
				equipe="Walawala"
				dosard={2}
				arrivee={writable(1245)}
			/>
		</TableBody>
	</Table>
</div>
