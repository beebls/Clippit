<script lang="ts">
	// @ts-ignore
	import FaVolumeUp from 'svelte-icons/fa/FaVolumeUp.svelte';
	// @ts-ignore
	import FaVolumeDown from 'svelte-icons/fa/FaVolumeDown.svelte';
	// @ts-ignore
	import FaVolumeMute from 'svelte-icons/fa/FaVolumeMute.svelte';
	import { currentProject } from '$lib/stores/currentProject';
	import { setGlobalVolume } from '$lib/audio/audioTest';

	let globalVolume: number = $currentProject.globalVolume * 100;

	function onGlobalVolumeChange(evt: any) {
		const globalVolume = evt.target.value / 100;
		$currentProject.globalVolume = globalVolume;
		setGlobalVolume(globalVolume);
	}
</script>

<div>
	<div
		class="group w-10 h-10 hover:w-full transition-all bg-primaryContainer-light dark:bg-primaryContainer-dark text-primary-light dark:text-primary-dark p-2 rounded-full flex gap-2 overflow-hidden"
	>
		<div class="h-full aspect-square" style={globalVolume > 50 ? 'transform: translateY(1px)' : ''}>
			{#if globalVolume > 50}
				<FaVolumeUp />
			{:else if globalVolume > 0}
				<FaVolumeDown />
			{:else}
				<FaVolumeMute />
			{/if}
		</div>
		<div class="flex items-center justify-center overflow-hidden">
			<input
				type="range"
				min={0}
				max={100}
				step={1}
				bind:value={globalVolume}
				on:change={onGlobalVolumeChange}
				class="group-hover:opacity-100 -translate-x-full group-hover:translate-x-0 opacity-0 transition-all"
			/>
		</div>
	</div>
</div>
