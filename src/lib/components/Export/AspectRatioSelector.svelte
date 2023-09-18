<script lang="ts">
	import OptionsWithCustomSelector from '../Primitives/OptionsWithCustomSelector.svelte';

	export let aspectRatio: number = 0;
	export let videoRef: HTMLVideoElement;
	let aspectRatioOptions = [
		{ value: 0, text: 'Original' },
		{ value: 16 / 9, text: '16 : 9' }
	];

	let videoAspect: number;

	if (videoRef) videoAspect = videoRef?.videoWidth / videoRef?.videoHeight;
</script>

{#if videoAspect < 16 / 9}
	<div class="flex flex-col items-center justify-center">
		<span class="font-subheading text-2xl">Aspect Ratio</span>
		<OptionsWithCustomSelector
			options={aspectRatioOptions}
			bind:chosenValue={aspectRatio}
			hasCustomOption={false}
		/>
	</div>
{/if}
