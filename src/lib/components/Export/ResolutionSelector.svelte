<script lang="ts">
	import OptionsWithCustomSelector from '../Primitives/OptionsWithCustomSelector.svelte';

	export let videoRef: HTMLVideoElement;
	export let chosenResolution: number = 0;
	let videoHeightOptions = [{ value: 0, text: 'Original' }];

	function calcHeightOptions() {
		let temp = [{ value: 0, text: 'Original' }];
		switch (true) {
			case videoRef.videoHeight > 2160:
				temp = [...temp, { value: 2160, text: '2160p' }, { value: 1080, text: '1080p' }];
				break;
			case videoRef.videoHeight > 1440:
				temp = [...temp, { value: 1440, text: '1440p' }, { value: 1080, text: '1080p' }];
				break;
			case videoRef.videoHeight > 1080:
				temp = [...temp, { value: 1080, text: '1080p' }, { value: 720, text: '720p' }];
				break;
			default:
				temp = [...temp, { value: 720, text: '720p' }, { value: 480, text: '480p' }];
				break;
		}

		videoHeightOptions = temp;
	}
	$: if (videoRef) calcHeightOptions();
</script>

<div class="flex flex-col items-center justify-center">
	<span class="font-subheading text-2xl">Resolution</span>
	<OptionsWithCustomSelector
		options={videoHeightOptions}
		bind:chosenValue={chosenResolution}
		customValueAnnotation="p"
	/>
</div>
