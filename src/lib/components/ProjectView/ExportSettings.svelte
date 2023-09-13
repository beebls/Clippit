<script lang="ts">
	import { onMount } from 'svelte';

	export let videoRef: HTMLVideoElement;
	let encoder: 'x264' | 'x265' = 'x264';
	let chosenVideoHeightOption: number = 0;
	let videoHeightOptions = [
		{ value: 0, text: 'Original' },
		{ value: 2, text: 'Original' },
		{ value: 3, text: 'Original' },
		{ value: 4, text: 'Original' }
	];

	let customHeight: String = '';

	function calcHeightOptions() {
		let temp = [{ value: 0, text: 'Original' }];
		if (videoRef.videoHeight >= 2160) {
			temp = [...temp, { value: 2160, text: '2160p' }];
		}
		if (videoRef.videoHeight >= 1440) {
			temp = [...temp, { value: 1440, text: '1440p' }];
		}
		if (videoRef.videoHeight >= 1080) {
			temp = [...temp, { value: 1080, text: '1080p' }];
		}
		if (videoRef.videoHeight >= 720) {
			temp = [...temp, { value: 720, text: '720p' }];
		}
		videoHeightOptions = temp;
	}
	$: if (videoRef) calcHeightOptions();
	function onCustomHeightChange() {
		if (Number(customHeight)) {
			chosenVideoHeightOption = -1;
		} else {
			chosenVideoHeightOption = 0;
		}
	}
	$: if (customHeight !== undefined) onCustomHeightChange();
</script>

<div class="flex flex-col justify-center items-center">
	<span class="font-fancy text-3xl">Export</span>
	<div
		class="flex rounded-full gap-0.5 bg-outline-light dark:bg-outline-dark border-2 border-outline-light dark:border-outline-dark overflow-hidden"
	>
		{#each videoHeightOptions as option, index}
			<button
				on:click={() => (chosenVideoHeightOption = option.value)}
				class={`${
					chosenVideoHeightOption === option.value
						? 'bg-secondaryContainer-light dark:bg-secondaryContainer-dark text-onSecondaryContainer-light dark:text-onSecondaryContainer-dark'
						: 'bg-containers-0-light dark:bg-containers-0-dark'
				} px-4 py-2`}>{option.text}</button
			>
		{/each}
		<div
			class={`${
				customHeight !== ''
					? 'bg-secondaryContainer-light dark:bg-secondaryContainer-dark text-onSecondaryContainer-light dark:text-onSecondaryContainer-dark'
					: 'bg-containers-0-light dark:bg-containers-0-dark'
			} px-4 py-2`}
		>
			<input
				class="bg-transparent w-12 text-center"
				type="string"
				bind:value={customHeight}
				on:input={(evt) => {
					console.log('test');
					const num = Number(evt.target.value);
					if (!num || num === 0) {
						customHeight = '';
						return;
					}
					customHeight = evt.target.value;
				}}
			/>
		</div>
	</div>
</div>
