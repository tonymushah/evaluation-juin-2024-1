<script lang="ts">
	import { TableBodyCell, TableBodyRow } from 'flowbite-svelte';
	export let coureur: string;
	export let points: number;
	$: isPoints = points > 1;
</script>

<TableBodyRow>
	<TableBodyCell>{coureur}</TableBodyCell>
	<TableBodyCell>
		{points}
		<span>
			{#if isPoints}
				points
			{:else}
				point
			{/if}
		</span>
	</TableBodyCell>
</TableBodyRow>
