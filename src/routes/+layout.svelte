<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import { generateMaterialPalette } from '$lib/utils/generateMaterialPalette';
	import ErrorModal from '$lib/components/ErrorModal.svelte';
	import TitleBar from '$lib/components/Native/TitleBar.svelte';
	import AppFrame from '$lib/components/Native/AppFrame.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	onMount(async () => {
		let systemAccentColor: number[] | string = await invoke('get_accent_color');
		if (systemAccentColor[3] === 0) {
			systemAccentColor = localStorage.getItem('materialYouBaseColor') ?? '#028090';
		}
		generateMaterialPalette(systemAccentColor);
	});
</script>

<AppFrame>
	<div
		class="bg-containers-0-light dark:bg-containers-0-dark text-black dark:text-white min-h-screen w-full flex flex-col overflow-hidden"
	>
		<TitleBar />
		<ErrorModal />
		<slot />
	</div>
</AppFrame>
