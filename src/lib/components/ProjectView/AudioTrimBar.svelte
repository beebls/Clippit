<script lang="ts">
	export let index: number;
	export let trackWidth: number;
	export let trackOffset: number;
	export let startLeft: number;

	import { currentProject } from '$lib/stores/currentProject';
</script>

<div class="relative bg-containers-2-light dark:bg-containers-2-dark rounded-lg">
	<div
		class="bg-secondary-light dark:bg-secondary-dark rounded-lg absolute h-full"
		style={`width: ${trackWidth + trackOffset * 2}px; left: calc(${startLeft}px); opacity: ${
			$currentProject.volumes?.[index] === 0 ? 0 : 1
		}`}
	/>
</div>
