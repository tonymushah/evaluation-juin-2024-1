<script lang="ts" context="module">
	import { derived, readonly, writable, type Readable, type Writable } from 'svelte/store';
	import { getContext } from 'svelte';
	import { query as joueursQuery } from '$lib/equipe/componnents/etape/joueurs.query';

	const pointsContextKey = 'ETAPE-SUB-POINT-KEY';
	export function getPointsContextKey(): Writable<number> {
		const context = getContext<Writable<number> | undefined>(pointsContextKey);
		if (context) {
			return context;
		} else {
			throw new Error(`The ${pointsContextKey} should be defined`);
		}
	}
	type SelectedJoueurMap = Map<number, boolean>;
	type SelectedJoueurStoreExtra = {
		select(coureur: number): Promise<void> | void;
		unselect(coureur: number): Promise<void> | void;
		listenToCoeur(coureur: number): Readable<boolean>;
		start(): void | Promise<void>;
	};
	type SelectedJoueurStore = Readable<SelectedJoueurMap> & SelectedJoueurStoreExtra;
	type SelectedJoueurStoreWritable = Writable<SelectedJoueurMap> & SelectedJoueurStoreExtra;

	const selectedJoueursContextKey = 'SELECT-JOUEUR-KEY';
	const addCoureurToEtapeMutation = graphql(`
		mutation addCoureurToEtape($etape: Int!, $joueur: Int!) {
			ajouterJoueurToEtape(etape: $etape, joueur: $joueur) {
				idTempsCoureur
				temps
				points
				equipeCoureur
				etape
			}
		}
	`);
	const removeCoureurToEtapeMutation = graphql(`
		mutation removeCoureurToEtape($etape: Int!, $joueur: Int!) {
			removeJoueurToEtape(etape: $etape, joueur: $joueur) {
				idTempsCoureur
				temps
				points
				equipeCoureur
				etape
			}
		}
	`);
	function initSelectedJoueurStore(client: Client, etape: number): SelectedJoueurStore {
		let isLoading = false;
		const write = writable<SelectedJoueurMap>(new Map(), (set, update) => {
			isLoading = true;
			const sub = client
				.query(joueursQuery, {
					etape
				})
				.subscribe((res) => {
					const joueurs = res.data?.etape.joueurs;
					if (joueurs) {
						update((map) => {
							joueurs.forEach((j) => {
								map.set(j.coureur, true);
							});
							return map;
						});
						isLoading = false;
					} else if (res.error) {
						isLoading = false;
					}
				});
			return () => {
				sub.unsubscribe();
			};
		});
		async function start() {
			if (!isLoading) {
				isLoading = true;
				await client
					.query(joueursQuery, {
						etape
					})
					.then((res) => {
						const joueurs = res.data?.etape.joueurs;
						if (joueurs) {
							write.update((map) => {
								joueurs.forEach((j) => {
									map.set(j.coureur, true);
								});
								return map;
							});
						}
					});
				isLoading = false;
			}
		}
		const read = readonly(write);
		return setContext(selectedJoueursContextKey, {
			async select(coureur) {
				if (!isLoading) {
					isLoading = true;
					await client
						.mutation(addCoureurToEtapeMutation, {
							joueur: coureur,
							etape
						})
						.toPromise()
						.finally(() => (isLoading = false));
					await this.start();
				}
			},
			async unselect(coureur) {
				if (!isLoading) {
					isLoading = true;
					await client
						.mutation(removeCoureurToEtapeMutation, {
							joueur: coureur,
							etape
						})
						.toPromise()
						.finally(() => (isLoading = false));
					await this.start();
				}
			},
			listenToCoeur(coureur) {
				return derived(write, (v) => v.get(coureur) ?? false);
			},
			start,
			...read
		});
	}
	export function getSelectedJoueurStore(): SelectedJoueurStore {
		const context = getContext<SelectedJoueurStore | undefined>(selectedJoueursContextKey);
		if (context) {
			return context;
		} else {
			throw new Error(`The ${selectedJoueursContextKey} should be defined`);
		}
	}
</script>

<script lang="ts">
	import type { LayoutServerData } from './$types';
	import { setContext } from 'svelte';
	import { getContextClient, type Client } from '@urql/svelte';
	import { graphql } from '$lib/equipe/gql';
	export let data: LayoutServerData;
	const client = getContextClient();
	const points = setContext(pointsContextKey, writable(data.points));
	const selecteds = initSelectedJoueurStore(client, data.rang);
</script>

<div class="grid grid-cols-2">
	<h2 class="text-xl font-semibold underline">{data.nom_etape}</h2>
	<p class="text-lg">
		Points: {$points}
		{#if $points > 1}
			points
		{:else}
			point
		{/if}
	</p>
</div>
