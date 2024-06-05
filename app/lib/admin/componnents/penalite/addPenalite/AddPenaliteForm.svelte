<script lang="ts">
	import { Button, Input, Label } from 'flowbite-svelte';
	import SelectEquipe from './equipes/SelectEquipe.svelte';
	import SelectEtape from './etape/SelectEtape.svelte';
</script>

<div class="flex items-center justify-center w-full h-full">
	<div class="flex items-center justify-center flex-col gap-4">
		<form method="post" action="?/add">
			<div class="w-full m-2">
				<Label>Equipe</Label>
				<SelectEquipe name="equipe" />
			</div>
			<div class="w-full m-2">
				<Label>Etapes</Label>
				<SelectEtape name="etape" />
			</div>
			<div class="w-full m-2">
				<Label>Temps</Label>
				<Input type="text" name="temps" />
			</div>
			<div class="w-full m-2">
				<Button type="submit">Ajouter</Button>
			</div>
		</form>
	</div>
</div>
