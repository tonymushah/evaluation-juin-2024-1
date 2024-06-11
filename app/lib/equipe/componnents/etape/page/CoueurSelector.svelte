<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { readable, type Readable } from 'svelte/store';

	export let nom: string;
	export let isSelected: Readable<boolean> = readable(false);
	const dispatch = createEventDispatcher<{
		click: MouseEvent & {
			currentTarget: EventTarget & HTMLDivElement;
		};
		keydown: KeyboardEvent & {
			currentTarget: EventTarget & HTMLDivElement;
		};
	}>();
</script>

<div
	class="flex rounded bg-slate-300 transition-colors hover:bg-slate-400 p-6 justify-center items-center"
	class:selected={$isSelected}
	role="button"
	on:click={(e) => {
		dispatch('click', e);
	}}
	tabindex="0"
	on:keydown={(e) => {
		dispatch('keydown', e);
	}}
>
	{nom}
</div>

<style lang="postcss">
	.selected {
		background-color: theme('colors.orange.300');
	}
	.selected:hover {
		background-color: theme('colors.orange.400');
	}
</style>
