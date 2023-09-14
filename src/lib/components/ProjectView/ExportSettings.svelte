<script lang="ts">
	import BitrateSelector from '../Export/BitrateSelector.svelte';
	import ExportButton from '../Export/ExportButton.svelte';
	export let videoRef: HTMLVideoElement;
	import ResolutionSelector from '../Export/ResolutionSelector.svelte';

	let resolution: number = 0;
	let fileSize: number = 0;
</script>

<div class="flex flex-col justify-center items-center px-2 gap-8">
	<span class="font-heading text-3xl">Export</span>
	<ResolutionSelector bind:videoRef bind:chosenResolution={resolution} />
	<BitrateSelector bind:chosenFilesize={fileSize} />
	<ExportButton bind:resolution bind:fileSize />
</div>
