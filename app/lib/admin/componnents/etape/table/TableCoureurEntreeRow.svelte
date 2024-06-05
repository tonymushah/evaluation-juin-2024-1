<script lang="ts">
	import { formatSecond } from '$lib';
	import { Input, TableBodyCell, TableBodyRow } from 'flowbite-svelte';
	export let coureur: string;
	export let equipe: string;
	export let dosard: number;
	export let arrivee: number | undefined = undefined;
	export let points: number | undefined = undefined;
	$: arrivee_output = arrivee ? formatSecond(arrivee) : undefined;
</script>

<TableBodyRow>
	<TableBodyCell>
		{coureur}
	</TableBodyCell>
	<TableBodyCell>
		{equipe}
	</TableBodyCell>
	<TableBodyCell>
		{dosard}
	</TableBodyCell>
	<TableBodyCell>
		{#if arrivee_output == undefined}
			<form method="post" action="?add_arrivee">
				<Input name="temps" type="text" />
				<input hidden name="dosard" value={dosard} />
			</form>
		{:else}
			{arrivee_output}
		{/if}
	</TableBodyCell>
	<TableBodyCell>
		{#if points}
			{points}
		{:else}
			Non definie
		{/if}
	</TableBodyCell>
</TableBodyRow>
