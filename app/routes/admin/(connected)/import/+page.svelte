<script>
	import { Button, Fileupload, Label } from 'flowbite-svelte';
</script>

<svelte:head>
	<title>Import de donnees</title>
</svelte:head>

<div class="flex w-full justify-center items-center gap-2 flex-col">
	<h2 class="text-2xl font-bold">Import de donnees</h2>
	<div class="flex flex-col w-50">
		<form method="post" action="?/points" enctype="multipart/form-data">
			<Label class="space-y-2 mb-2">
				<span>Points</span>
			</Label>
			<div class="flex flex-row gap-3">
				<input type="file" class="rounded border" name="file" />
				<Button type="submit">Importer</Button>
			</div>
		</form>
	</div>
	<div class="flex flex-col w-50">
		<form method="post" action="?/etapes" enctype="multipart/form-data">
			<Label class="space-y-2 mb-2">
				<span>Etapes</span>
			</Label>
			<div class="flex flex-row gap-3">
				<input type="file" class="rounded border" name="file" />
				<Button type="submit">Importer</Button>
			</div>
		</form>
	</div>
	<div class="flex flex-col w-50">
		<form method="post" action="?/resultats" enctype="multipart/form-data">
			<Label class="space-y-2 mb-2">
				<span>Resultats</span>
			</Label>
			<div class="flex flex-row gap-3">
				<input type="file" class="rounded border" name="file" />
				<Button type="submit">Importer</Button>
			</div>
		</form>
	</div>
</div>
