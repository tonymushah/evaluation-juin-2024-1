<script lang="ts">
	import { brand } from '$lib';
	import IndexNavbar from '$lib/global/componnents/IndexNavbar.svelte';
</script>

<IndexNavbar />

<svelte:head>
	<title>{brand}</title>
</svelte:head>

<div class="content ml-5 mr-5 mt-5">
	<slot />
</div>
