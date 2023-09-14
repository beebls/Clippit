<script lang="ts">
	import { goto } from '$app/navigation';
	import { currentProject } from '$lib/stores/currentProject';
	import { generateMaterialPalette } from '$lib/utils/generateMaterialPalette';
	import { open } from '@tauri-apps/api/dialog';
	async function startProject() {
		const selected: string = (await open({
			filters: [{ name: 'Video', extensions: ['mp4'] }]
		})) as string;
		$currentProject.fileName = selected;
		goto('/projectView');
	}
</script>

<div class="w-full h-full flex-grow flex flex-col items-center justify-center relative">
	<button
		on:click={startProject}
		class="bg-primaryContainer-light dark:bg-primaryContainer-dark text-onPrimaryContainer-light dark:text-onPrimaryContainer-dark text-4xl p-4 rounded-xl"
		>Get File</button
	>
	<div class="absolute bottom-4">
		<input
			type="color"
			on:change={(evt) => {
				generateMaterialPalette(evt.target.value);
			}}
		/>
	</div>
</div>
