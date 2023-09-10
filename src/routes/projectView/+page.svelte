<script lang="ts">
	import AudioElem from '$lib/components/AudioElem.svelte';

	import { onDestroy, onMount } from 'svelte';
	import {
		pause as pauseAudio,
		play as playAudio,
		seek as seekAudio,
		reset as resetAudio
	} from '../../lib/audio/audioTest';
	import { invoke } from '@tauri-apps/api';
	import { save } from '@tauri-apps/api/dialog';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { join, appCacheDir, basename } from '@tauri-apps/api/path';
	import { currentProject } from '$lib/stores/currentProject';
	import VideoTrimBar from '$lib/components/ProjectView/VideoTrimBar.svelte';
	import PlayButton from '$lib/components/ProjectView/PlayButton.svelte';

	let audioSrces: any[] = [];

	let projectHash: string;

	onMount(() => {
		startProject();
	});

	onDestroy(() => {
		resetAudio();
	});

	let trimmedFileName: string;
	async function trimFileName() {
		trimmedFileName = await basename($currentProject.fileName);
	}

	async function startProject() {
		trimFileName();
		const [num_audio_tracks, projectId]: [number, string] = await invoke('start_project', {
			fileName: $currentProject.fileName
		});
		projectHash = projectId;
		const cacheRoot = await appCacheDir();
		const tempRoot = await join(cacheRoot, 'temp');
		const projectRoot = await join(tempRoot, projectId);
		const vidFile = await join(projectRoot, 'video.mp4');
		const audioFiles = Array(num_audio_tracks)
			.fill('')
			.map((e, i) => `track_${i}.mp3`);

		audioFiles.forEach(async (e, i) => {
			const filePath = await join(projectRoot, e);
			const assetUrl = convertFileSrc(filePath);
			volumes[i] = 100;
			audioSrces = [...audioSrces, assetUrl];
		});

		const assetUrl = convertFileSrc(vidFile);

		videoSrc = document.createElement('source');
		videoSrc.type = 'video/mp4';
		videoSrc.src = assetUrl;
		videoRef.appendChild(videoSrc);
		videoRef.load();
	}

	let videoRef: HTMLVideoElement;
	let videoLoaded: boolean = false;
	let videoSrc: HTMLSourceElement;

	let playing: boolean = false;

	let duration: number = 0;

	let startTime: number, endTime: number;

	const volumes: number[] = [];

	function onVideoLoad() {
		videoLoaded = true;
		duration = videoRef.duration;
	}

	function play() {
		videoRef.play();
		playAudio();
		playing = true;
	}

	function pause() {
		videoRef.pause();
		pauseAudio();
		playing = false;
	}

	function seek(time: number) {
		videoRef.currentTime = time;
		seekAudio(time);
	}

	async function beginExport() {
		const height = Number(prompt('Name the new height of the video (0 for original): ')) || 0;
		const fileSize = Number(prompt('Name the new file size for the video (0 for original): ')) || 0;
		const encoder = Number(prompt('Choose the encoder (0 for x265, 1 for x264)')) || 0;
		const outputPath = await save({ filters: [{ name: 'Video', extensions: ['mp4'] }] });
		invoke('export_project', {
			projectHash: projectHash,
			startTime: startTime,
			endTime: endTime,
			audioVolumes: volumes.map((e) => e / 100),
			outputFile: outputPath,
			newHeight: height,
			newMegabytes: fileSize,
			encoderType: encoder === 1 ? 'x264' : 'x265'
		});
	}

	let trackWidth: number, trackOffset: number, startLeft: number;
</script>

<div
	class="flex flex-col flex-grow overflow-hidden gap-4"
	style={`display: ${videoLoaded ? 'flex' : 'none'}; height: calc(100vh - 2rem);`}
>
	<!-- svelte-ignore a11y-media-has-caption -->
	<div class="flex-grow p-4">
		<div class="relative w-full h-full bg-containers-2-light dark:bg-containers-2-dark">
			<video
				class="absolute right-0 bottom-0 min-w-full min-h-full w-auto h-auto bg-cover overflow-hidden"
				style="height: 100%"
				bind:this={videoRef}
				on:loadeddata={onVideoLoad}
				on:click={() => (playing ? pause() : play())}
			/>
		</div>
	</div>

	{#if videoLoaded}
		<div class="flex flex-col bg-containers-2-light dark:bg-containers-2-dark">
			<div class="h-12 flex items-center pl-4 gap-2">
				<PlayButton {play} {pause} {playing} />
				<PlayButton {play} {pause} {playing} />
				<span class="ml-auto pr-4">0:12/1:00</span>
			</div>
			<div class="grid grid-cols-[1fr,7fr] grid-rows-1 p-4 pt-0">
				<div
					class="w-full grid grid-cols-1 gap-4 p-4 pl-0"
					style={`grid-template-rows: 3rem repeat(${audioSrces.length}, 1.5rem)`}
				>
					<div
						class="flex items-center justify-center bg-containers-4-light dark:bg-containers-4-dark rounded-2xl"
					>
						<span>{trimmedFileName}</span>
					</div>
					{#each audioSrces as src, index}
						<AudioElem {src} {index} bind:volume={volumes[index]} />
					{/each}
				</div>
				<div
					class="w-full grid grid-cols-1 gap-4 p-4 bg-containers-0-light dark:bg-containers-0-dark rounded-3xl"
					style={`grid-template-rows:  3rem repeat(${audioSrces.length}, 1.5rem)`}
				>
					<VideoTrimBar
						{duration}
						bind:trackWidth
						bind:trackOffset
						bind:startTime
						bind:startLeft
						bind:endTime
						{pause}
						bind:playing
						{seek}
						bind:videoRef
					/>
					{#each audioSrces as src, index}
						<div class="relative bg-containers-2-light dark:bg-containers-2-dark rounded-lg">
							<div
								class="bg-secondary-light dark:bg-secondary-dark rounded-lg absolute h-full"
								style={`width: ${
									trackWidth + trackOffset * 2
								}px; left: calc(${startLeft}px); opacity: ${volumes[index] === 0 ? 0 : 1}`}
							/>
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/if}

	<!-- <div class="flex p-4 bg-containers-2-light dark:bg-containers-2-dark gap-4">
			<div class="flex flex-col py-4 gap-4">
				<div
					class="flex items-center justify-center h-12 w-full bg-containers-4-light dark:bg-containers-4-dark rounded-2xl"
				>
					<span>{trimmedFileName}</span>
				</div>

				{#each audioSrces as src, index}
					<AudioElem {src} {index} bind:volume={volumes[index]} />
				{/each}
			</div>
			<div class="w-full bg-containers-0-light dark:bg-containers-0-dark p-4 rounded-xl">
				<VideoTrimBar
					{duration}
					bind:startTime
					bind:endTime
					{pause}
					bind:playing
					{seek}
					bind:videoRef
				/>
			</div>
		</div> -->
	<button on:click={beginExport}>Export</button>
</div>
