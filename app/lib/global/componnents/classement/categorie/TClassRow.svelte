<script lang="ts">
	import { formatSecond } from '$lib';
	import { TableBodyCell, TableBodyRow } from 'flowbite-svelte';
	export let coureur: string;
	export let points: number;
	export let temps: number;
	export let bg: string = 'inherit';
	$: temps_ = formatSecond(temps);
	$: isPoints = points > 1;
</script>

<TableBodyRow style="background-color : {bg};">
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
	<TableBodyCell>
		{temps_}
	</TableBodyCell>
</TableBodyRow>
