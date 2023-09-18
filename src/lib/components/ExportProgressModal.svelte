<script lang="ts">
	import { currentProject } from '$lib/stores/currentProject';
	import { appCacheDir, join } from '@tauri-apps/api/path';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import ProgressModal from './Primitives/ProgressModal.svelte';
	export let open: boolean = true;

	let filePath: string;
	let assetUrl: string;

	async function getFilePath() {
		if (!$currentProject.projectHash) return;
		const cacheRoot = await appCacheDir();
		const tempRoot = await join(cacheRoot, 'temp');
		const projectRoot = await join(tempRoot, $currentProject.projectHash);
		const outputRoot = await join(projectRoot, 'output');
		const path = await join(outputRoot, 'export_progress.json');
		filePath = path;
		createUrl();
	}

	async function createUrl() {
		const url = convertFileSrc(filePath);
		assetUrl = url;
	}

	$: if ($currentProject.projectHash && !assetUrl) getFilePath();
</script>

{#if $currentProject.isExporting}
	<ProgressModal bind:assetUrl bind:open title="Exporting" />
{/if}
