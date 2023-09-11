<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import '../app.css';
	import { page } from '$app/stores';
	import { generateMaterialPalette } from '$lib/utils/generateMaterialPalette';
	import ErrorModal from '$lib/components/ErrorModal.svelte';
	onMount(() => {
		generateMaterialPalette(localStorage.getItem('materialYouBaseColor') ?? '#028090');
	});
</script>

<div
	class="bg-containers-0-light dark:bg-containers-0-dark text-black dark:text-white min-h-screen w-full flex flex-col overflow-hidden"
>
	<ErrorModal />
	<nav class="h-8">
		<button on:click={() => goto('/')}>go back</button>
	</nav>
	<slot />
</div>
