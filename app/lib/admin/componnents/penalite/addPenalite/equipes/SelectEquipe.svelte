<script lang="ts">
	export let name: string;
	import { Select, type SelectOptionType } from 'flowbite-svelte';
	import getClassement from './query';
	import { onMount } from 'svelte';
	import { derived } from 'svelte/store';

	const { data, hasNext, isLoading, reset, next } = getClassement();
	onMount(async () => {
		await reset();
	});
	$: {
		if ($hasNext) {
			if (!$isLoading) {
				next();
			}
		}
	}
	const items = derived<typeof data, SelectOptionType<string>[]>(data, (d) =>
		d.map((i) => ({
			value: i.id,
			name: i.nom
		}))
	);
</script>

<Select placeholder="Choisiser equipe" items={$items} {name} />
