<script lang="ts">
	import { goto } from '$app/navigation';
	import { currentProject, defaultProject } from '$lib/stores/currentProject';
	import { generateMaterialPalette } from '$lib/utils/generateMaterialPalette';
	import { open } from '@tauri-apps/api/dialog';
	import FileDrop from 'svelte-tauri-filedrop';

	async function startProject(filePath: string) {
		$currentProject = { ...defaultProject };
		if (!filePath) return;
		$currentProject.fileName = filePath;
		goto('/projectView');
	}

	async function onGetButtonClick() {
		const selected: string = (await open({
			filters: [{ name: 'Video', extensions: ['mp4'] }]
		})) as string;
		startProject(selected);
	}
	function onDrop(evt: string[]) {
		console.log(evt);
		if (evt && evt[0]) {
			startProject(evt[0]);
		}
	}

	function onColorChange(evt: any) {
		generateMaterialPalette(evt?.target?.value);
	}
</script>

<div class="w-full h-full flex-grow flex flex-col items-center justify-center relative">
	<FileDrop extensions={['mp4']} handleFiles={onDrop} let:files>
		<div
			class="dropzone w-full h-full flex-grow flex flex-col gap-2 items-center justify-center"
			class:droppable={files.length > 0}
		>
			<button
				on:click={onGetButtonClick}
				class="bg-primaryContainer-light dark:bg-primaryContainer-dark text-onPrimaryContainer-light dark:text-onPrimaryContainer-dark text-4xl p-4 rounded-xl"
			>
				Get File
			</button>
			<span>Or drag and drop</span>
		</div>
	</FileDrop>

	<div class="absolute bottom-4">
		<input type="color" on:change={onColorChange} />
	</div>
</div>
