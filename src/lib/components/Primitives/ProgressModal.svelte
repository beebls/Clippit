<script lang="ts">
	import type { ProgressFile } from '$lib/types/rustInteropTypes';
	import { errors } from '$lib/stores/errorStore';
	import { AlertDialog } from 'radix-svelte';
	import { onMount } from 'svelte';
	export let open: boolean;

	export let title: string = 'Loading';

	let text: string = 'Extracting Video';
	let progNumber: number;

	export let assetUrl: string;

	async function fetchProgress() {
		if (!assetUrl) {
			setTimeout(() => {
				fetchProgress();
			}, 10);
		} else {
			fetch(assetUrl)
				.then((res) => {
					return res.json();
				})
				.then((json: ProgressFile) => {
					progNumber = json?.number;
					text = json?.text;

					if (!text.toLowerCase().includes('done')) {
						setTimeout(() => {
							fetchProgress();
						}, 10);
					}
				});
		}
	}

	onMount(() => fetchProgress());
</script>

{#if $errors.length === 0}
	<AlertDialog.Root bind:open>
		<AlertDialog.Portal>
			<AlertDialog.Overlay
				class="fixed inset-0 bg-black/50 data-[state=open]:animate-overlayShow"
			/>
			<AlertDialog.Content
				class="fixed left-[50%] top-[50%] max-h-[85vh] w-[90vw] max-w-[450px] translate-x-[-50%] 
			translate-y-[-50%] rounded-md p-[25px] shadow-lg
			focus:outline-none data-[state=open]:animate-contentShow bg-containers-4-light dark:bg-containers-4-dark text-white"
			>
				<AlertDialog.Title class="text-center text-3xl">{title}</AlertDialog.Title>
				<AlertDialog.Description class="pt-4">
					{#if text}
						<span>{text}</span>
					{/if}
				</AlertDialog.Description>

				<div class="absolute bottom-0 left-0 w-full h-2">
					<div
						class="w-full h-full bg-containers-5-light dark:bg-containers-5-dark relative isolate overflow-hidden rounded-b-md before:absolute before:inset-0 before:-translate-x-full before:animate-[shimmer_2s_infinite] loading-bar"
					/>
				</div>
				<AlertDialog.Action />
			</AlertDialog.Content>
		</AlertDialog.Portal>
	</AlertDialog.Root>
{/if}
