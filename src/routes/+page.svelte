<script lang="ts">
	import { goto } from '$app/navigation';
	import { currentProject } from '$lib/stores/currentProject';
	import { open } from '@tauri-apps/api/dialog';
	async function startProject() {
		const selected: string = (await open({
			filters: [{ name: 'Video', extensions: ['mp4'] }]
		})) as string;
		currentProject.set({ fileName: selected });
		goto('/projectView');
	}
</script>

<div class="w-full h-full min-h-screen flex items-center justify-center">
	<button on:click={startProject} class="bg-zinc-800 text-4xl p-4 rounded-xl">Get File</button>
</div>
