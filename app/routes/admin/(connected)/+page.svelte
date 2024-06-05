<script lang="ts">
	import { brand } from '$lib';
	import TableHomeEtapes from '$lib/admin/componnents/etape/TableHomeEtapes.svelte';
</script>

<svelte:head>
	<title>Admin | {brand}</title>
</svelte:head>

<h2 class="text-2xl mb-3 font-bold underline">Liste des etapes</h2>

<TableHomeEtapes />
