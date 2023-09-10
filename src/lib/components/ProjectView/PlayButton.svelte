<script lang="ts">
	import FaPlay from 'svelte-icons/fa/FaPlay.svelte';
	import FaPause from 'svelte-icons/fa/FaPause.svelte';
	export let play: () => void;
	export let pause: () => void;
	export let playing: boolean;
</script>

<button
	class="w-10 h-10 bg-primaryContainer-light dark:bg-primaryContainer-dark text-primary-light dark:text-primary-dark p-3 rounded-full"
	on:click={() => (playing ? pause() : play())}
>
	{#if playing}
		<div class="w-full h-full">
			<FaPause />
		</div>
	{:else}
		<div class="w-full h-full translate-x-[1px]">
			<FaPlay />
		</div>
	{/if}
</button>
