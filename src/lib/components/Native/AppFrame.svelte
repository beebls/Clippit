<script lang="ts">
	import { clippit } from '$lib/stores/appStore';
	import { appWindow } from '@tauri-apps/api/window';
	import { onDestroy, onMount } from 'svelte';
	let unlisten: () => void;

	onMount(async () => {
		unlisten = await appWindow.onResized(async () => {
			$clippit.maximized = await appWindow.isMaximized();
		});
		appWindow.listen('tauri://update-status', (evt) => {
			console.log(evt);
		});
	});

	onDestroy(() => {
		unlisten();
	});
</script>

<div
	class={`absolute inset-0 overflow-hidden ${
		$clippit.maximized
			? 'rounded-none'
			: 'rounded-lg border-x-[1px] border-b-[1px] border-[#2f2f2f]'
	}`}
>
	<slot />
</div>
