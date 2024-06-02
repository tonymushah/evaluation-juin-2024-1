<script lang="ts">
	import TableEtapeRow from '$lib/admin/componnents/etape/TableEtapeRow.svelte';
	import { Table, TableBody, TableHead, TableHeadCell } from 'flowbite-svelte';
</script>

<Table color="blue">
	<TableHead>
		<TableHeadCell>Rang</TableHeadCell>
		<TableHeadCell>Nom etape</TableHeadCell>
		<TableHeadCell>Kilometrage</TableHeadCell>
		<TableHeadCell>Date Debut</TableHeadCell>
		<TableHeadCell>Date Fin</TableHeadCell>
	</TableHead>
	<TableBody tableBodyClass="divide-y">
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
		<TableEtapeRow rang={1} nom="Some Nom" kilometrage={12.4} dateDebut={new Date('2024-06-01')} />
	</TableBody>
</Table>
