<script lang="ts">
	import TablePenalite from '$lib/admin/componnents/penalite/TablePenalite.svelte';
	import TablePenaliteHead from '$lib/admin/componnents/penalite/TablePenaliteHead.svelte';
	import AddPenaliteForm from '$lib/admin/componnents/penalite/addPenalite/AddPenaliteForm.svelte';
	import { TabItem, Table, TableBody, Tabs } from 'flowbite-svelte';
</script>

<svelte:head>
	<title>Penalites</title>
</svelte:head>

<h2 class="text-2xl font-bold">Penalites</h2>

<Tabs>
	<TabItem open title="Listes des penalites">
		<TablePenalite />
	</TabItem>
	<TabItem title="Ajouter penalite">
		<AddPenaliteForm />
	</TabItem>
</Tabs>
