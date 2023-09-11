<script lang="ts">
	import { goto } from '$app/navigation';
	import { errors } from '$lib/stores/errorStore';
	import { AlertDialog } from 'radix-svelte';
	let open: boolean = false;
	errors.subscribe((errors) => {
		if (errors.length > 0) {
			open = true;
		} else {
			open = false;
		}
	});

	function onDismissError() {
		goto('/');
		$errors = [];
	}
</script>

<AlertDialog.Root bind:open>
	<AlertDialog.Portal>
		<AlertDialog.Overlay class="fixed inset-0 bg-black/50 data-[state=open]:animate-overlayShow" />
		<AlertDialog.Content
			class="fixed left-[50%] top-[50%] max-h-[85vh] w-[90vw] max-w-[450px] translate-x-[-50%] 
			translate-y-[-50%] rounded-md p-[25px] shadow-lg
			focus:outline-none data-[state=open]:animate-contentShow bg-containers-4-light dark:bg-containers-4-dark text-white"
		>
			<AlertDialog.Title class="text-center text-3xl">Error!</AlertDialog.Title>
			<AlertDialog.Description class="py-4">
				<ul class={$errors.length > 1 ? 'list-disc ml-4' : 'list-none'}>
					{#each $errors as error}
						<li class="">{error}</li>
					{/each}
				</ul>
			</AlertDialog.Description>
			<AlertDialog.Action>
				<button on:click={() => onDismissError()} class="text-primary-light dark:text-primary-dark"
					>Go Back</button
				>
			</AlertDialog.Action>
		</AlertDialog.Content>
	</AlertDialog.Portal>
</AlertDialog.Root>
