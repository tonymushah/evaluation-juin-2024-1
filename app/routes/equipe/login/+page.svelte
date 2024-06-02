<script lang="ts">
	import { Label, Input, Button } from 'flowbite-svelte';
</script>

<div class="flex w-screen h-screen items-center justify-center">
	<div class="w-2/3 h-2/3 rounded-md bg-slate-300 dark:bg-slate-700 p-4 grid grid-cols-2">
		<div class="flex items-center justify-center w-full h-full">
			<img src="/logo1.png" alt="logo" />
		</div>
		<div class="flex w-full h-full items-center justify-center flex-col gap-5">
			<h2 class="dark:text-white text-2xl font-bold">Se Connecter</h2>
			<form method="post" class=" w-11/12 flex flex-col gap-5">
				<div>
					<Label class="mb-2" for="pseudo">Pseudo de l'equipe</Label>
					<Input name="nom" type="text" id="pseudo" placeholder="Vautour" required />
				</div>
				<div>
					<Label class="mb-2" for="password">Mot de passe</Label>
					<Input name="password" type="text" id="password" placeholder="Password" required />
				</div>
				<Button type="submit">Se Connecter</Button>
			</form>
		</div>
	</div>
</div>
