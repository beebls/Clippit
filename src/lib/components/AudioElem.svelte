<script lang="ts">
	import { connectElement } from '$lib/audio/audioTest';
	import { generateRandomString } from '$lib/utils/generateRandomString';
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';

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
</script>

<div class="flex bg-containers-4-light dark:bg-containers-4-dark rounded-xl flex-col h-12">
	<audio
		id={alphabet[index]}
		bind:this={elem}
		{src}
		controls={false}
		unselectable="on"
		crossorigin="anonymous"
	/>
	<h1>Track {index + 1}</h1>
	<div class="flex">
		<input type="range" min="0" max="100" step="1" on:change={onSliderChange} bind:value={volume} />
	</div>
</div>
