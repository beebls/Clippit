<script lang="ts">
	import { currentProject } from '$lib/stores/currentProject';
	import { invoke } from '@tauri-apps/api';
	import { save } from '@tauri-apps/api/dialog';

	export let resolution: number;
	export let fileSize: number;
	export let encoder: 'x264' | 'x265' = 'x264';

	async function beginExport() {
		const outputPath = await save({ filters: [{ name: 'Video', extensions: ['mp4'] }] });
		invoke('export_project', {
			projectHash: $currentProject.projectHash,
			startTime: $currentProject.startTime,
			endTime: $currentProject.endTime,
			audioVolumes: $currentProject.volumes.map((e) => e / 100),
			outputFile: outputPath,
			newHeight: resolution,
			newMegabytes: fileSize,
			encoderType: encoder
		});
	}
</script>

<button
	on:click={beginExport}
	class="bg-primaryContainer-light dark:bg-primaryContainer-dark text-onPrimaryContainer-light dark:text-onPrimaryContainer-dark p-4 rounded-2xl w-32 font-2xl font-semibold"
	>Export
</button>
