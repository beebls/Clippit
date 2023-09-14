<script lang="ts">
	import OptionsWithCustomSelector from '../Primitives/OptionsWithCustomSelector.svelte';

	export let chosenFilesize: number = 0;
	let fileSizeOptions = [
		{ value: 0, text: 'Original' },
		{ value: 8, text: '10MB' },
		{ value: 25, text: '25MB' }
	];
</script>

<div class="flex flex-col items-center justify-center">
	<span class="font-subheading text-2xl">File Size</span>
	<OptionsWithCustomSelector
		options={fileSizeOptions}
		bind:chosenValue={chosenFilesize}
		customValueAnnotation="MB"
	/>
</div>
