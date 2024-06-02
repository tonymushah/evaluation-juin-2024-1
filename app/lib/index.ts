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
