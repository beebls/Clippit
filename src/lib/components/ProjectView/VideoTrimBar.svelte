<script lang="ts">
	import { onMount } from 'svelte';
	import type { MouseEventHandler } from 'svelte/elements';

	export let duration: number;

	let trackRef: HTMLDivElement;
	let startRef: HTMLDivElement;
	let endRef: HTMLDivElement;

	export let startLeft: number = 0;
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
		videoRef.currentTime = startTime;
		calculatePlayHeadLocation(true);
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

	export let trackWidth: number;
	export let trackOffset: number;

	let playHeadLeftPercent: number;

	onMount(() => {
		let start = startRef?.getBoundingClientRect();
		trackWidth = trackRef?.getBoundingClientRect().width - start.width * 2;
		trackOffset = start.width;
		window.addEventListener('resize', afterDragPercentCalculation);
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

		trackWidth = track.width * rangePercent - end.width;
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
		playHeadLeftPercent = videoPercent;
		if (videoRef.currentTime > endTime) {
			pause();
			seek(startTime);
			calculatePlayHeadLocation(true);
		}
		setTimeout(() => {
			calculatePlayHeadLocation();
		}, 10);
	}
	$: if (playing) calculatePlayHeadLocation();
</script>

<div
	bind:this={trackRef}
	class="w-full h-12 relative rounded-xl bg-containers-6-light dark:bg-containers-6-dark"
>
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		on:click={onTrackClick}
		class="bg-primaryContainer-light dark:bg-primaryContainer-dark border-4 border-l-0 border-r-0 border-primary-light dark:border-primary-dark h-12 absolute top-0"
		style={`width: ${trackWidth + 6}px; left: calc(${startLeft}px + ${trackOffset}px)`}
	/>
	<div
		bind:this={startRef}
		class="bg-primary-light dark:bg-primary-dark w-4 h-full absolute top-0 cursor-pointer rounded-l-xl flex items-center justify-end"
		style={`left: ${startLeft}px`}
		on:pointerdown={onStartDown}
		on:pointerup={onStartUp}
	>
		<div
			class="bg-primaryContainer-dark h-10 float-right w-2 border-r-0 rounded-xl translate-x-1/2"
		/>
	</div>

	<div
		bind:this={endRef}
		class="bg-primary-light dark:bg-primary-dark w-4 h-full absolute top-0 cursor-pointer rounded-r-xl flex items-center"
		style={`right: ${endRight}px`}
		on:pointerdown={onEndDown}
		on:pointerup={onEndUp}
	>
		<div
			class="bg-primaryContainer-dark h-10 float-right w-2 border-r-0 rounded-xl -translate-x-1/2"
		/>
	</div>
	<div
		class="w-1 h-12 absolute top-0 z-50 pointer-events-none"
		style={`left: ${playHeadLeftPercent}%`}
	>
		<div class="relative w-full h-full bg-secondary-light dark:bg-secondary-dark">
			<div
				class="absolute left-1/2 -translate-x-1/2 w-0 -top-1 z-50 h-0 border-l-8 border-r-8 border-t-8 border-l-transparent border-r-transparent border-t-secondary-light dark:border-t-secondary-dark"
			/>
		</div>
	</div>
</div>
