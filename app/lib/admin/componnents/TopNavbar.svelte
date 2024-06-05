<script lang="ts">
	import { route } from '$lib/ROUTES';
	import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Button } from 'flowbite-svelte';
</script>

<Navbar let:NavContainer color="primary">
	<NavContainer class="border px-5 py-2 rounded-lg bg-white dark:bg-gray-600">
		<NavBrand href={route('/admin')}>
			<img src="/logo2.png" class="me-3 h-6 sm:h-9" alt="Flowbite Logo" />
			<span class="self-center whitespace-nowrap text-xl font-semibold">Admin</span>
		</NavBrand>
		<NavHamburger />
		<NavUl>
			<NavLi href={route('/admin/penalite')}>Penalites</NavLi>
			<NavLi class="items-center justify-center" href={route('/admin/import')}>Imports</NavLi>
			<NavLi href={route('/admin/reset-db')}>Reset Database</NavLi>
		</NavUl>
	</NavContainer>
</Navbar>
