import { writable } from 'svelte/store';

export interface CurrentProject {
	fileName: string;
	projectHash: string;
	startTime: number;
	endTime: number;
	duration: number;
	volumes: number[];
	globalVolume: number;
	isExporting: boolean;
}

export const defaultProject = {
	fileName: '',
	projectHash: '',
	startTime: 0,
	endTime: 0,
	duration: 0,
	globalVolume: 1,
	volumes: [],
	isExporting: false
};

export const currentProject = writable<CurrentProject>({ ...defaultProject });
