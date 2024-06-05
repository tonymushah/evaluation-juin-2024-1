<script lang="ts" context="module">
</script>

<script lang="ts">
	import { Button, Table, TableBody } from 'flowbite-svelte';
	import TClassHead from './TClassHead.svelte';
	import TClassRow from './TClassRow.svelte';
	import getClassement from './query';
	import { derived } from 'svelte/store';
	export let categorie: string;
	$: c = getClassement(categorie);
	$: data = c.data;
	$: colors = derived(data, (d_) => {
		let r = structuredClone(d_);
		r.sort((a, b) => a.points - b.points);
		let re: [number, number][] = [];

		let current = null;
		let cnt = 0;
		for (let i = 0; i < r.length; i++) {
			if (r[i] != current) {
				if (cnt > 0) {
					re.push([current!.points, cnt]);
				}
				current = r[i];
				cnt = 1;
			} else {
				cnt++;
			}
		}
		if (cnt > 0) {
			re.push([current!.points, cnt]);
		}
		return new Map(
			re.map(([p, c]) => [p, `#${Math.floor(Math.random() * 16777215).toString(16)}`])
		);
	});
	$: hasNext = c.hasNext;
	$: isLoading = c.isLoading;
	$: reset = c.reset;
	$: next = c.next;
	$: {
		reset();
	}
</script>

{#if $isLoading}
	<h3>Loading...</h3>
{/if}

<Button disabled={$isLoading} on:click={reset}>Reset</Button>

<Table>
	<TClassHead />
	<TableBody tableBodyClass="divide-y divide-x">
		{#each $data as class_}
			<TClassRow
				bg={$colors.get(class_.points)}
				coureur={class_.coureur.toFixed()}
				points={class_.points}
				temps={class_.temps}
			/>
		{/each}
	</TableBody>
</Table>

{#if $hasNext && !$isLoading}
	<div class="flex w-full items-center justify-center">
		<Button
			disabled={$isLoading}
			on:click={async () => {
				await next();
			}}>Next</Button
		>
	</div>
{/if}
