<script lang="ts">
	import { goto } from '$app/navigation';
	import { route } from '$lib/ROUTES';
	import { onMount } from 'svelte';

	onMount(() => {
		goto(route('/equipe/login'));
	});
</script>
