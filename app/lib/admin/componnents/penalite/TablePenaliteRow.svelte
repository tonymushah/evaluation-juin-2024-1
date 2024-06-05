<script lang="ts">
	import { TableBodyCell, TableBodyRow } from 'flowbite-svelte';
	export let etape: string;
	export let equipe: string;
	export let temps: string;
</script>

<TableBodyRow>
	<TableBodyCell>{etape}</TableBodyCell>
	<TableBodyRow>{equipe}</TableBodyRow>
	<TableBodyCell>{temps}</TableBodyCell>
	<TableBodyCell>
		<slot name="fourth-col" />
	</TableBodyCell>
</TableBodyRow>
