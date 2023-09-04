<script lang="ts">
	import AudioElem from '$lib/components/AudioElem.svelte';

	import { onMount } from 'svelte';
	import {
		pause as pauseAudio,
		play as playAudio,
		seek as seekAudio
	} from '../../lib/audio/audioTest';
	import { invoke } from '@tauri-apps/api';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { join, appCacheDir } from '@tauri-apps/api/path';
	import { currentProject } from '$lib/stores/currentProject';
	import VideoTrimBar from '$lib/components/ProjectView/VideoTrimBar.svelte';

	let audioSrces: any[] = [];

	let projectHash: string;

	onMount(() => {
		startProject();
	});

	async function startProject() {
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
	$: console.log(volumes);
</script>

<div class="flex flex-col" style={`display: ${videoLoaded ? 'flex' : 'none'}`}>
	<!-- svelte-ignore a11y-media-has-caption -->
	<video
		bind:this={videoRef}
		style="width: 100%; height: 400px;"
		on:loadeddata={onVideoLoad}
		on:click={() => (playing ? pause() : play())}
	/>
	{#if videoLoaded}
		<VideoTrimBar
			{duration}
			bind:startTime
			bind:endTime
			{pause}
			bind:playing
			{seek}
			bind:videoRef
		/>
		<button class="bg-zinc-800 h-12 w-12 rounded-full" on:click={() => (playing ? pause() : play())}
			>{playing ? 'Pause' : 'Play'}</button
		>
		{#each audioSrces as src, index}
			<AudioElem {src} {index} bind:volume={volumes[index]} />
		{/each}
	{/if}
	<button
		on:click={() => {
			invoke('export_project', {
				projectHash: projectHash,
				startTime: startTime,
				endTime: endTime,
				audioVolumes: volumes.map((e) => e / 100)
			});
		}}>Test</button
	>
</div>
