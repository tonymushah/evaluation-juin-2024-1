<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import EtapeCard from './EtapeCard.svelte';

	export let etapes: ComponentProps<EtapeCard>[] = [];
</script>

<div class="flex flex-col mt-4">
	{#each etapes as etape}
		<EtapeCard {...etape} />
	{:else}
		<h3 class="text-md">Rien a voir ici</h3>
	{/each}
</div>
