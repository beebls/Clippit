<script lang="ts">
	import { currentProject } from '$lib/stores/currentProject';
	import { appCacheDir, join } from '@tauri-apps/api/path';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import ProgressModal from './Primitives/ProgressModal.svelte';
	export let open: boolean = true;

	let filePath: string;
	let assetUrl: string;

	async function getFilePath() {
		if (!$currentProject.projectHash) {
			return;
		}
		const cacheRoot = await appCacheDir();
		const tempRoot = await join(cacheRoot, 'temp');
		const projectRoot = await join(tempRoot, $currentProject.projectHash);
		const path = await join(projectRoot, 'import_progress.json');
		filePath = path;
		createUrl();
	}

	async function createUrl() {
		const url = convertFileSrc(filePath);
		assetUrl = url;
	}

	$: if ($currentProject.projectHash) getFilePath();
</script>

<ProgressModal bind:assetUrl bind:open />
