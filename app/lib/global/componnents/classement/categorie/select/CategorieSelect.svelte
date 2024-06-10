<script lang="ts">
	export let value: string | undefined = undefined;
	import { Select, type SelectOptionType } from 'flowbite-svelte';
	import getEtapeClassement from './query';
	import { onMount } from 'svelte';
	import { derived, get } from 'svelte/store';

	const { data, hasNext, isLoading, reset, next } = getEtapeClassement();
	onMount(async () => {
		await reset();
		const first = get(data)[0];
		if (first) value = first.id;
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

<Select placeholder="Choisiser equipe" items={$items} bind:value />
