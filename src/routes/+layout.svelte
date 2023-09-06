<script lang="ts">
	import { goto } from '$app/navigation';
	import '../app.css';
</script>

<div
	class="bg-zinc-300 dark:bg-zinc-700 text-black dark:text-white min-h-screen w-screen max-w-screen flex flex-col overflow-hidden"
>
	<nav class="h-8">
		<button on:click={() => goto('/')}>go back</button>
	</nav>
	<slot />
</div>
