<script lang="ts">
	import { route } from '$lib/ROUTES';
	import { Badge, Card } from 'flowbite-svelte';
	import type { ComponentProps } from 'svelte';
	import JoueurEtapeCard from './JoueurEtapeCard.svelte';
	export let id: string;
	export let etat: string;
	export let nom: string;
	export let points: number = 0;
	export let joueurs: ComponentProps<JoueurEtapeCard>[] = [];
</script>

<div class="flex bg-slate-300 rounded flex-col">
	<div class="flex flex-row justify-center items-center gap-2">
		<Badge>{etat}</Badge>
		<a href={route('/equipe/etape/[id]', { id })} class="hover:text-orange-400 transition-colors">
			{nom}
		</a>
		<p>
			Points: {points}
		</p>
	</div>
	<div class="flex flex-wrap gap-2">
		{#each joueurs as joueur}
			<JoueurEtapeCard {...joueur} />
		{/each}
	</div>
</div>
