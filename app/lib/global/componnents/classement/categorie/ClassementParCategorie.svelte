<script lang="ts">
	import { Label } from 'flowbite-svelte';
	import CategorieSelect from './select/CategorieSelect.svelte';
	import TableClassmmentCategorie from './TableClassmmentCategorie.svelte';

	let categorie: string | undefined = undefined;
</script>

<div class="flex flex-row gap-2 items-center">
	<Label>Categories</Label>
	<CategorieSelect bind:value={categorie} />
</div>

{#if categorie}
	<TableClassmmentCategorie bind:categorie />
{:else}
	<h2 class="text-xl font-bold">Please select a categorie</h2>
{/if}
