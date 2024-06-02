<script lang="ts">
	import { TableBodyCell, TableBodyRow } from 'flowbite-svelte';

	export let rang: number;
	export let nom: string;
	export let kilometrage: number;
	export let dateDebut: Date;
	export let dateFin: Date | undefined = undefined;
</script>

<TableBodyRow class="hover:bg-slate-800 active:bg-slate-900 transition-colors">
	<TableBodyCell>
		{rang}
	</TableBodyCell>
	<TableBodyCell>
		{nom}
	</TableBodyCell>
	<TableBodyCell>
		{kilometrage}
	</TableBodyCell>
	<TableBodyCell>
		{dateDebut.toDateString()}
	</TableBodyCell>
	<TableBodyCell>
		{#if dateFin}
			{dateFin.toDateString()}
		{:else}
			Non definie
		{/if}
	</TableBodyCell>
</TableBodyRow>
