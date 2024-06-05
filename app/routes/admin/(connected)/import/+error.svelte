<script lang="ts">
	import { page } from '$app/stores';
	import { brand } from '$lib';
</script>

<svelte:head>
	<title>Error | {brand}</title>
</svelte:head>

<h1 class="text-2xl w-full h-full flex justify-center items-center">{$page.error?.message}</h1>
