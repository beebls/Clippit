<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window';
	import { clippit } from '$lib/stores/appStore';
	import VscChromeClose from '../Icons/VscChromeClose.svelte';
	import VscChromeMaximize from '../Icons/VscChromeMaximize.svelte';
	import VscChromeMinimize from '../Icons/VscChromeMinimize.svelte';
</script>

<div
	data-tauri-drag-region
	class={`z-[2147483647] w-full text-black dark:text-white top-0 left-0 right-0 flex bg-containers-0-light dark:bg-containers-0-dark overflow-hidden border-t-[1px] border-[#2f2f2f] ${
		$clippit.maximized ? 'rounded-t-none' : 'rounded-t-lg'
	}`}
>
	<div class="grid place-content-center pl-2">
		<a
			class="text-xl titlebar-logo-text hover:scale-95 hover:active:scale-90 transition-all"
			href="/"
		>
			Clippit
		</a>
	</div>
	<div class="ml-auto flex items-center justify-center">
		<button
			on:click={() => appWindow.minimize()}
			class="h-8 w-[47.5px] p-2 hover:bg-[#3f3f3f] hover:active:bg-[#333333]"
		>
			<VscChromeMinimize />
		</button>
		<button
			on:click={() => appWindow.toggleMaximize()}
			class="h-8 w-[47.5px] p-2 hover:bg-[#3f3f3f] hover:active:bg-[#333333]"
		>
			<VscChromeMaximize />
		</button>
		<button
			on:click={() => appWindow.close()}
			class="h-8 w-[47.5px] p-2 hover:bg-[#fe0000] hover:active:bg-[#fe0000aa]"
		>
			<VscChromeClose />
		</button>
	</div>
</div>
