import { getContextClient, type Client, type CombinedError } from '@urql/svelte';
import { writable, type Readable, type Writable } from 'svelte/store';

// place files you want to import through the `$lib` alias in this folder.
export const brand = 'Utlimate Team Race';

export const tempsRegex = /(?<heures>[\d+]*):(?<minutes>[\d]*):(?<secondes>[\d]*)/gm;

export function formatSecond(a: number) {
	const seconde = a % 60;
	const b = (a - seconde) / 60;
	const minutes = b % 60;
	const heures = (b - minutes) / 60;
	return `${heures}:${minutes}:${seconde}`;
}

export function toSecond(heures: number, minutes: number, secondes: number) {
	return heures * 3600 + minutes * 60 + secondes;
}

export type PaginatedData<T> = {
	data: Readable<Array<T>>;
	next: () => void | Promise<void>;
	reset: () => void | Promise<void>;
	hasNext: Readable<boolean>;
	isLoading: Readable<boolean>;
	error: Readable<CombinedError | undefined>;
	obs: IntersectionObserver;
};

export type PaginatedDataInit<T> = {
	data: Writable<Array<T>>;
	hasNext: Writable<boolean>;
	isLoading: Writable<boolean>;
	error: Writable<CombinedError | undefined>;
	client: Client;
};

export function get_paginated_init_data<T>(): PaginatedDataInit<T> {
	const client = getContextClient();
	const data = writable<T[]>([]);
	const isLoading = writable(false);
	const hasNext = writable(true);
	return {
		client,
		data,
		isLoading,
		hasNext,
		error: writable()
	};
}
