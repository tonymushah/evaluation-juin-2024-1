<script lang="ts" context="module">
	import { derived, get, readonly, writable, type Readable, type Writable } from 'svelte/store';
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
		const isLoading = writable(false);
		const write = writable<SelectedJoueurMap>(new Map(), (set, update) => {
			client
				.query(joueursQuery, {
					etape
				})
				.toPromise()
				.then((res) => {
					const joueurs = res.data?.etape.joueurs;
					if (joueurs) {
						update((map) => {
							joueurs.forEach((j) => {
								map.set(j.coureur, true);
							});
							return map;
						});
					}
				})
				.catch((e) => {
					console.error(e);
				});
		});
		async function start() {
			console.log('start');
			if (!get(isLoading)) {
				isLoading.set(true);
				await client
					.query(joueursQuery, {
						etape
					})
					.toPromise()
					.then((res) => {
						const joueurs = res.data?.etape.joueurs;
						console.log(joueurs);
						if (joueurs) {
							write.update((map) => {
								joueurs.forEach((j) => {
									map.set(j.coureur, true);
								});
								return map;
							});
						}
					})
					.finally(() => isLoading.set(false))
					.catch((e) => {
						console.error(e);
					});
			}
		}
		const read = readonly(write);
		return setContext(selectedJoueursContextKey, {
			async select(coureur) {
				console.log('click');
				if (!get(isLoading)) {
					isLoading.set(true);
					await client
						.mutation(addCoureurToEtapeMutation, {
							joueur: coureur,
							etape
						})
						.toPromise()
						.then((e) => {
							if (e.error) {
								console.log(e.error);
							}
							return e;
						})
						.finally(() => isLoading.set(false))
						.then(() => start())
						.catch((e) => {
							console.error(e);
						});
				}
			},
			async unselect(coureur) {
				const is = get(isLoading);
				if (!is) {
					isLoading.set(true);
					console.log('load');
					await client
						.mutation(removeCoureurToEtapeMutation, {
							joueur: coureur,
							etape
						})
						.toPromise()
						.then((e) => {
							if (e.error) {
								console.log(e.error);
							}
							return e;
						})
						.finally(() => isLoading.set(false))

						.then(() => start())
						.catch((e) => {
							console.error(e);
						});
				}
			},
			listenToCoeur(coureur) {
				return derived(read, (v) => {
					const res = v.get(coureur);
					if (typeof res == 'boolean') {
						return res;
					} else {
						return false;
					}
				});
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

<svelte:head>
	<title>Etape {data.nom_etape}</title>
</svelte:head>

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

<slot />
