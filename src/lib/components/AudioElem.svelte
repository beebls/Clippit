<script lang="ts">
	import { connectElement } from '$lib/audio/audioTest';
	import { onMount } from 'svelte';
	// @ts-ignore
	import FaVolumeUp from 'svelte-icons/fa/FaVolumeUp.svelte';
	// @ts-ignore
	import FaVolumeDown from 'svelte-icons/fa/FaVolumeDown.svelte';
	// @ts-ignore
	import FaVolumeMute from 'svelte-icons/fa/FaVolumeMute.svelte';

	const alphabet = 'abcdefghijklmnopqrstuvwxyz';

	export let src: any;
	export let index: number;
	export let volume: number;

	let elem: HTMLAudioElement;
	let gainNode: GainNode;

	onMount(() => {
		const newGainNode = connectElement(`#${alphabet[index]}`);
		gainNode = newGainNode;
		elem.load();
	});

	function onSliderChange(evt: any) {
		const gain = evt.target.value / 100;
		gainNode.gain.setValueAtTime(gain, gainNode.context.currentTime + 0.001);
	}

	let volumeControlOpen: boolean = false;
</script>

<div class="flex bg-containers-4-light dark:bg-containers-4-dark rounded-md relative">
	<audio
		id={alphabet[index]}
		bind:this={elem}
		{src}
		controls={false}
		unselectable="on"
		crossorigin="anonymous"
	/>
	<span class="ml-2">Track {index + 1}</span>
	<span class="ml-auto">{volume}%</span>
	<button
		class="h-full aspect-square p-1 ml-4 bg-containers-5-light dark:bg-containers-5-dark rounded-md"
		on:click={() => (volumeControlOpen = !volumeControlOpen)}
	>
		{#if volume > 50}
			<FaVolumeUp />
		{:else if volume > 0}
			<FaVolumeDown />
		{:else}
			<FaVolumeMute />
		{/if}
	</button>
	{#if volumeControlOpen}
		<div
			class="absolute z-[1000] -bottom-full left-full -translate-x-1/2 bg-containers-6-light dark:bg-containers-6-dark rounded-full h-5"
		>
			<input
				class="mx-4"
				type="range"
				min="0"
				max="100"
				step="1"
				on:change={onSliderChange}
				bind:value={volume}
			/>
		</div>
	{/if}
</div>
