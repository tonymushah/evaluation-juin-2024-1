<script lang="ts">
	import { formatSecond, tempsRegex, toSecond } from '$lib';
	import { Input, TableBodyCell, TableBodyRow } from 'flowbite-svelte';
	import { derived, writable, type Writable } from 'svelte/store';
	export let coureur: string;
	export let equipe: string;
	export let dosard: number;
	export let arrivee: Writable<number | undefined> = writable(undefined);
	export let points: Writable<number | undefined> = writable(undefined);
	$: arrivee_output = derived(arrivee, (a) => {
		if (a) {
			return formatSecond(a);
		}
	});
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
		{#if $arrivee_output == undefined}
			<form
				on:submit|preventDefault|stopPropagation={(e) => {
					const data = new FormData(e.currentTarget);
					const temps = data.get('temps');
					if (typeof temps == 'string') {
						const tmpRegRes = tempsRegex.exec(temps);
						if (tmpRegRes != null) {
							const heures = Number(tmpRegRes.groups?.heures);
							const minutes = Number(tmpRegRes.groups?.minutes);
							const secondes = Number(tmpRegRes.groups?.secondes);
							if (isFinite(heures) && isFinite(minutes) && isFinite(secondes)) {
								arrivee.set(toSecond(heures, minutes, secondes));
								return;
							}
						}
						const temps_num = Number(temps);
						if (isFinite(temps_num)) {
							arrivee.set(temps_num);
							return;
						}
					}
				}}
			>
				<Input name="temps" type="text" />
			</form>
		{:else}
			{$arrivee_output}
		{/if}
	</TableBodyCell>
	<TableBodyCell>
		{#if $points}
			{$points}
		{:else}
			Non definie
		{/if}
	</TableBodyCell>
</TableBodyRow>
