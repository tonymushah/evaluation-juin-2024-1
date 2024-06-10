<script lang="ts">
	import { route } from '$lib/ROUTES';
	import { Badge, Card } from 'flowbite-svelte';
	import type { ComponentProps } from 'svelte';
	import JoueurEtapeCard from './JoueurEtapeCard.svelte';
	import getEtapeCoueur from './joueurs.query';
	import { derived } from 'svelte/store';
	export let id: number;
	export let etat: string;
	export let nom: string;
	$: queryRes = getEtapeCoueur(id);
	$: joueurs = derived(
		queryRes,
		(js) =>
			js.data?.etape.joueurs.map<ComponentProps<JoueurEtapeCard> & { id: number }>((j) => ({
				id: j.coureur,
				nom: j.nomCoureur,
				points: typeof j.points == 'number' ? j.points : 0
			})) ?? []
	);
	$: points = derived(joueurs, (js) => js.map((j) => j.points ?? 0).reduce((ac, c) => ac + c, 0));
</script>

<div class="flex bg-slate-300 rounded flex-col w-full">
	<div class="flex flex-row pl-2 items-center gap-2 text-xl">
		<Badge>{etat}</Badge>
		<a href={route('/equipe/etape/[id]', { id })} class="hover:text-orange-700 transition-colors">
			{nom}
		</a>
		<p>
			Points: {$points}
		</p>
	</div>
	<div class="flex flex-wrap gap-2 m-1">
		{#each $joueurs as joueur}
			<JoueurEtapeCard {...joueur} />
		{:else}
			<h3 class="text-md">Pas de joueur selectionne</h3>
		{/each}
	</div>
</div>
