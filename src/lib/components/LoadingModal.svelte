<script lang="ts">
	import { AlertDialog } from 'radix-svelte';
	export let open: boolean = false;
</script>

<AlertDialog.Root bind:open>
	<AlertDialog.Portal>
		<AlertDialog.Overlay class="fixed inset-0 bg-black/50 data-[state=open]:animate-overlayShow" />
		<AlertDialog.Content
			class="fixed left-[50%] top-[50%] max-h-[85vh] w-[90vw] max-w-[450px] translate-x-[-50%] 
			translate-y-[-50%] rounded-md p-[25px] shadow-lg
			focus:outline-none data-[state=open]:animate-contentShow bg-containers-4-light dark:bg-containers-4-dark text-white"
		>
			<AlertDialog.Title class="text-center text-3xl">Loading</AlertDialog.Title>
			<AlertDialog.Description class="py-4" />
			<AlertDialog.Action />
		</AlertDialog.Content>
	</AlertDialog.Portal>
</AlertDialog.Root>
