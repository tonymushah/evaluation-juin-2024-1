<script lang="ts" context="module">
	export type ClassementItem = {
		coureur: string;
		points: number;
	};
</script>

<script lang="ts">
	import { Table, TableBody } from 'flowbite-svelte';
	import TClassHead from './TClassHead.svelte';
	import TClassRow from './TClassRow.svelte';
	import { readable, type Readable } from 'svelte/store';

	export let classement: Readable<Array<ClassementItem>> = readable([]);
</script>

<Table>
	<TClassHead />
	<TableBody tableBodyClass="divide-y divide-x">
		{#each $classement as class_ (class_.coureur)}
			<TClassRow {...class_} />
		{/each}
	</TableBody>
</Table>
