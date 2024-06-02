<script lang="ts">
	import { Button, NavBrand, NavHamburger, NavLi, NavUl, Navbar } from 'flowbite-svelte';
	import { ChevronDownOutline } from 'flowbite-svelte-icons';

	import { page } from '$app/stores';
	import { route } from '$lib/ROUTES';
	$: activeUrl = $page.url.pathname;
</script>

<Navbar rounded class="bg-slate-300 dark:bg-slate-800">
	<NavBrand href="/">
		<img class="h-12" src="/logo2.png" alt="Ultimate Team Race" />
	</NavBrand>
	<div class="flex md:order-2">
		<Button size="sm">Se connecter</Button>
		<NavHamburger />
	</div>
	<h1 class="font-brand text-orange-600 font-bold text-4xl">Ultimate Team Race</h1>
</Navbar>
