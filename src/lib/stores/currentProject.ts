import { writable } from 'svelte/store';

export interface CurrentProject {
	fileName: string;
}

export const currentProject = writable({ fileName: '' });
