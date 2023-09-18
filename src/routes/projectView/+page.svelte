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
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { join, appCacheDir, basename } from '@tauri-apps/api/path';
	import { currentProject } from '$lib/stores/currentProject';
	import VideoTrimBar from '$lib/components/ProjectView/VideoTrimBar.svelte';
	import PlayButton from '$lib/components/ProjectView/PlayButton.svelte';
	import { secondsToMinuteString } from '$lib/utils/secondsToMinuteString';
	import { errors } from '$lib/stores/errorStore';
	import ExportSettings from '$lib/components/ProjectView/ExportSettings.svelte';
	import AudioTrimBar from '$lib/components/ProjectView/AudioTrimBar.svelte';
	import ImportProgressModal from '$lib/components/ImportProgressModal.svelte';
	import GlobalVolumeButton from '$lib/components/ProjectView/GlobalVolumeButton.svelte';

	let audioSrces: any[] = [];

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
		if (!$currentProject.fileName) {
			$errors = [...$errors, 'No filename provided by selection'];
			return;
		}

		trimFileName();
		try {
			const [num_audio_tracks, projectId]: [number, string] = await invoke('start_project', {
				fileName: $currentProject.fileName
			});
			$currentProject.projectHash = projectId;
			const cacheRoot = await appCacheDir();
			const tempRoot = await join(cacheRoot, 'temp');
			const projectRoot = await join(tempRoot, projectId);
			const vidFile = await join(projectRoot, 'video.mp4');
			const audioFiles = Array(num_audio_tracks)
				.fill('')
				.map((e, i) => `track_${i}.mp3`);

			$currentProject.volumes = Array(audioFiles.length).fill(100);
			audioFiles.forEach(async (e, i) => {
				const filePath = await join(projectRoot, e);
				const assetUrl = convertFileSrc(filePath);
				audioSrces = [...audioSrces, assetUrl];
			});

			const assetUrl = convertFileSrc(vidFile);

			videoSrc = document.createElement('source');
			videoSrc.type = 'video/mp4';
			videoSrc.src = assetUrl;
			videoRef.appendChild(videoSrc);
			videoRef.load();
		} catch (err: any) {
			console.error('ERROR', err);
			$errors = [err, ...$errors];
		}
	}

	let videoRef: HTMLVideoElement;
	let videoLoaded: boolean = false;
	let videoSrc: HTMLSourceElement;

	let playing: boolean = false;

	let approxCurrentTime: number;

	function getTimeWhilePlaying(override: boolean = false) {
		if (!playing && !override) return;
		approxCurrentTime = videoRef.currentTime;
		setTimeout(() => {
			getTimeWhilePlaying();
		}, 100);
	}

	$: if (playing) getTimeWhilePlaying();

	function onVideoLoad() {
		videoLoaded = true;
		$currentProject.startTime = 0;
		$currentProject.endTime = videoRef.duration;
		$currentProject.duration = videoRef.duration;
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
		getTimeWhilePlaying();
	}

	let trackWidth: number = 0,
		trackOffset: number = 0,
		startLeft: number = 0;
</script>

<div class="flex-grow flex">
	<div
		class="flex flex-col w-full gap-4"
		style={`display: ${videoLoaded ? 'flex' : 'none'}; height: calc(100vh - 2rem);`}
	>
		<!-- svelte-ignore a11y-media-has-caption -->
		<div class="flex-grow">
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
		{#if !videoLoaded}
			<ImportProgressModal />
		{/if}

		{#if videoLoaded}
			<div class="flex flex-col bg-containers-2-light dark:bg-containers-2-dark">
				<div class="h-12 flex items-center pl-4 gap-2">
					<PlayButton {play} {pause} {playing} />
					<GlobalVolumeButton />
					<span class="ml-auto pr-4"
						>{secondsToMinuteString(approxCurrentTime)}/{secondsToMinuteString(
							Math.round($currentProject.duration)
						)}</span
					>
				</div>
				<div class="flex flex-row grid-rows-1 p-4 pt-0">
					<div
						class="grid grid-cols-1 gap-4 p-4 pl-0 min-w-[12rem]"
						style={`grid-template-rows: 3rem repeat(${audioSrces.length}, 1.5rem)`}
					>
						<div
							class="flex items-center justify-center bg-containers-4-light dark:bg-containers-4-dark rounded-2xl"
						>
							<span class="overflow-ellipsis whitespace-nowrap overflow-hidden max-w-[12rem] px-2"
								>{trimmedFileName}</span
							>
						</div>
						{#each audioSrces as src, index}
							<AudioElem {src} {index} bind:volume={$currentProject.volumes[index]} />
						{/each}
					</div>
					<div
						class="w-full flex-1 grid grid-cols-1 gap-4 p-4 bg-containers-0-light dark:bg-containers-0-dark rounded-3xl"
						style={`grid-template-rows:  3rem repeat(${audioSrces.length}, 1.5rem)`}
					>
						<VideoTrimBar
							bind:trackWidth
							bind:trackOffset
							bind:startLeft
							{pause}
							bind:playing
							{seek}
							bind:videoRef
						/>
						{#each audioSrces as test, index}
							<AudioTrimBar {index} bind:trackOffset bind:trackWidth bind:startLeft />
						{/each}
					</div>
				</div>
			</div>
		{/if}
	</div>
	<div>
		<ExportSettings bind:videoRef />
	</div>
</div>
