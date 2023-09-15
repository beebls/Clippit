<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window';
	import VscChromeClose from '../Icons/VscChromeClose.svelte';
	import VscChromeMaximize from '../Icons/VscChromeMaximize.svelte';
	import VscChromeMinimize from '../Icons/VscChromeMinimize.svelte';
</script>

<div
	data-tauri-drag-region
	class="z-[2147483647] flex bg-containers-0-light dark:bg-containers-0-dark"
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
