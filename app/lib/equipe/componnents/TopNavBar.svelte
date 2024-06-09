<script lang="ts">
	import { route } from '$lib/ROUTES';
	import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Button } from 'flowbite-svelte';
	import getCurrent from '../utils/queries/currentEquipe';
	const currentEquipe = getCurrent();
	$: nom = $currentEquipe.data?.current.nom;
</script>

<Navbar let:NavContainer color="primary">
	<NavContainer class="border px-5 py-2 rounded-lg bg-white dark:bg-gray-600">
		<NavBrand href={route('/equipe')}>
			<img src="/logo2.png" class="me-3 h-6 sm:h-9" alt="Flowbite Logo" />
			<span class="self-center whitespace-nowrap text-xl font-semibold">
				{#if nom}
					Equipe {nom}
				{:else}
					Equipe
				{/if}
			</span>
		</NavBrand>
		<NavHamburger />
		<NavUl>
			<NavLi>Classements</NavLi>
			<NavLi>Joueurs</NavLi>
			<NavLi href={'/equipe/logout'}>Se deconnecter</NavLi>
		</NavUl>
	</NavContainer>
</Navbar>
