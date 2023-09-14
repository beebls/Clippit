import { writable } from 'svelte/store';

export interface CurrentProject {
	fileName: string;
	projectHash: string;
	startTime: number;
	endTime: number;
	duration: number;
	volumes: number[];
	globalVolume: number;
}

export const currentProject = writable<CurrentProject>({
	fileName: '',
	projectHash: '',
	startTime: 0,
	endTime: 0,
	duration: 0,
	globalVolume: 1,
	volumes: []
});
