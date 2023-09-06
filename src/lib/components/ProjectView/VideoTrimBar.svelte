<script lang="ts">
	import { onMount } from 'svelte';
	import type { MouseEventHandler } from 'svelte/elements';

	export let duration: number;

	let trackRef: HTMLDivElement;
	let startRef: HTMLDivElement;
	let endRef: HTMLDivElement;

	let startLeft: number = 0;
	let endRight: number = 0;

	export let startTime: number = 0;
	export let endTime: number = duration;

	export let playing: boolean;
	export let pause: () => void;
	export let seek: (time: number) => void;

	export let videoRef: HTMLVideoElement;

	function onStartDown() {
		if (playing) pause();
		window.addEventListener('pointermove', onMoveWhenStartSelected);
		window.addEventListener('pointerup', onStartUp);
	}
	function onStartUp() {
		window.removeEventListener('pointermove', onMoveWhenStartSelected);
		window.removeEventListener('pointerup', onStartUp);
	}
	function onEndDown() {
		if (playing) pause();
		window.addEventListener('pointermove', onMoveWhenEndSelected);
		window.addEventListener('pointerup', onEndUp);
	}
	function onEndUp() {
		window.removeEventListener('pointermove', onMoveWhenEndSelected);
		window.removeEventListener('pointerup', onEndUp);
	}
	function onMoveWhenStartSelected(evt: PointerEvent) {
		const mouseX = evt.clientX;
		// const trackWidth = trackRef.getBoundingClientRect().width;
		const elemBox = startRef.getBoundingClientRect();
		const track = trackRef.getBoundingClientRect();

		const adjustedLeft = mouseX - track.left;
		if (mouseX < track.left) return;
		if (adjustedLeft + track.left + elemBox.width > endRef.getBoundingClientRect().left) return;
		startLeft = adjustedLeft;
		afterDragPercentCalculation();
	}
	function onMoveWhenEndSelected(evt: PointerEvent) {
		const mouseX = evt.clientX;
		// const trackWidth = trackRef.getBoundingClientRect().width;
		const elemBox = endRef.getBoundingClientRect();
		const track = trackRef.getBoundingClientRect();

		const adjustedRight = track.right - mouseX;

		if (mouseX > track.right) return;
		if (
			track.width - adjustedRight + track.left - elemBox.width <
			startRef.getBoundingClientRect().right
		)
			return;
		endRight = adjustedRight;
		afterDragPercentCalculation();
	}

	let middleWidth: number;
	let middleOffset: number;

	let playHeadLeftPercent: number;

	onMount(() => {
		let start = startRef?.getBoundingClientRect();
		middleWidth = trackRef?.getBoundingClientRect().width - start.width * 2;
		middleOffset = start.width;
	});

	function afterDragPercentCalculation() {
		const track = trackRef.getBoundingClientRect();
		const start = startRef.getBoundingClientRect();
		const end = endRef.getBoundingClientRect();
		const startMiddle = start.right - start.width / 2;
		const endMiddle = end.right - end.width / 2;

		const leftPercent = (startMiddle - track.left) / track.width;
		const rightPercent = (endMiddle - track.left) / track.width;

		const rangePercent = rightPercent - leftPercent;

		// This rounds down, to ensure you dont go OVER the original video duration;
		startTime = Math.round(duration * leftPercent * 100) / 100;
		endTime = Math.round(duration * rightPercent * 100) / 100;

		if (videoRef.currentTime < startTime || videoRef.currentTime > endTime) {
			seek(startTime);
			calculatePlayHeadLocation(true);
		}

		middleWidth = track.width * rangePercent - end.width;
	}
	function onTrackClick(evt: MouseEvent) {
		const mouseX = evt.clientX;
		const track = trackRef.getBoundingClientRect();
		const adjustedMousePos = mouseX - track.left;
		const mousePercent = (adjustedMousePos / track.width) * 100;
		const timeClickedOn = Math.floor(duration * (mousePercent / 100) * 100) / 100;

		playHeadLeftPercent = mousePercent;
		seek(timeClickedOn);
	}

	function calculatePlayHeadLocation(override: boolean = false) {
		if (!playing && !override) return;
		const videoPercent = (videoRef.currentTime / duration) * 100;
		console.log(videoPercent);
		playHeadLeftPercent = videoPercent;
		setTimeout(() => {
			calculatePlayHeadLocation();
		}, 10);
	}
	$: if (playing) calculatePlayHeadLocation();
</script>

<div bind:this={trackRef} class="bg-zinc-900 w-full h-12 relative rounded-xl">
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		on:click={onTrackClick}
		class="bg-[#192326] border-2 border-[#67c2ae] h-12 absolute top-0"
		style={`width: ${middleWidth}px; left: calc(${startLeft}px + ${middleOffset}px)`}
	/>
	<div
		bind:this={startRef}
		class="bg-[#67c2ae] w-2 h-full absolute top-0 cursor-pointer rounded-l-xl"
		style={`left: ${startLeft}px`}
		on:pointerdown={onStartDown}
		on:pointerup={onStartUp}
	/>
	<div
		bind:this={endRef}
		class="bg-[#67c2ae] w-2 h-full absolute top-0 cursor-pointer rounded-r-xl"
		style={`right: ${endRight}px`}
		on:pointerdown={onEndDown}
		on:pointerup={onEndUp}
	/>
	<div class="w-2 h-12 bg-white absolute top-0 z-50" style={`left: ${playHeadLeftPercent}%`} />
</div>
