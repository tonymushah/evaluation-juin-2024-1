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
	import { readable, writable, type Readable } from 'svelte/store';
	import { crossfade } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	const [send, receive] = crossfade({
		duration: 400,
		easing: quintOut
	});
	export let classement: Readable<Array<ClassementItem>> = readable([]);
</script>

<Table>
	<TClassHead />
	<TableBody tableBodyClass="divide-y divide-x">
		{#each $classement as class_ (class_.coureur)}
			<div class="content" in:send={{ key: class_.coureur }} out:receive={{ key: class_.coureur }}>
				<TClassRow {...class_} />
			</div>
		{/each}
	</TableBody>
</Table>
