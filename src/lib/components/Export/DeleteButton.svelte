<script lang="ts">
	import { goto } from '$app/navigation';
	import { currentProject } from '$lib/stores/currentProject';
	import { errors } from '$lib/stores/errorStore';
	import { invoke } from '@tauri-apps/api';

	async function beginDelete() {
		await invoke('remove_project', {
			fileName: $currentProject.fileName,
			projectHash: $currentProject.projectHash
		}).catch((err) => {
			$errors = [...$errors, err];
		});
		goto('/');
	}
</script>

<button
	on:click={beginDelete}
	class="bg-errorContainer-light dark:bg-errorContainer-dark text-onErrorContainer-light dark:text-onErrorContainer-dark p-4 rounded-2xl w-32 font-2xl font-semibold"
	>Delete
</button>
