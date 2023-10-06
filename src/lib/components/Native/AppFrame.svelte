<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window';
	import { onDestroy, onMount } from 'svelte';
	let maximized = false;
	let unlisten: () => void;

	onMount(async () => {
		unlisten = await appWindow.onResized(async () => {
			maximized = await appWindow.isMaximized();
		});
	});

	onDestroy(() => {
		unlisten();
	});
</script>

<div
	class={`absolute inset-0 overflow-hidden ${
		maximized ? 'rounded-none' : 'rounded-lg border-x-[1px] border-b-[1px] border-[#2f2f2f]'
	}`}
>
	<slot />
</div>
