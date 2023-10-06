import { writable } from 'svelte/store';

export interface Clippit {
	maximized: boolean;
}

export const clippit = writable<Clippit>({ maximized: false });
