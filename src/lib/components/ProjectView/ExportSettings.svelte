<script lang="ts">
	import AspectRatioSelector from '../Export/AspectRatioSelector.svelte';
	import BitrateSelector from '../Export/BitrateSelector.svelte';
	import DeleteButton from '../Export/DeleteButton.svelte';
	import ExportButton from '../Export/ExportButton.svelte';
	export let videoRef: HTMLVideoElement;
	import ResolutionSelector from '../Export/ResolutionSelector.svelte';
	import ExportProgressModal from '../ExportProgressModal.svelte';

	let resolution: number = 0;
	let fileSize: number = 0;
	let aspectRatio: number = 0;
</script>

<div class="flex flex-col justify-center items-center px-2 gap-8">
	<ExportProgressModal />
	<span class="font-heading text-3xl">Export</span>
	<ResolutionSelector bind:videoRef bind:chosenResolution={resolution} />
	<BitrateSelector bind:chosenFilesize={fileSize} />
	<AspectRatioSelector bind:aspectRatio bind:videoRef />
	<ExportButton bind:resolution bind:fileSize bind:aspectRatio />
	<DeleteButton />
</div>
