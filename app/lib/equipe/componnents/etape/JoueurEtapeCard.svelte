<script lang="ts">
	export let nom: string;
	export let points: number = 0;
</script>

<div class="flex bg-cyan-200 dark:bg-cyan-800 rounded p-3 gap-2 flex-col items-center">
	<h4 class=" font-mono text-md">{nom}</h4>
	<p>Points: {points}</p>
</div>
