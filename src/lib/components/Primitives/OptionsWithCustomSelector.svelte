<script lang="ts">
	export let options: { value: any; text: string; subText?: string }[];

	export let chosenValue = 0;
	export let customValueAnnotation: string = '';

	let chosenOption: number = 0;

	let usingCustomOption: boolean = false;

	let customOption: String = '';

	function onCustomInput(evt: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		// @ts-ignore
		const num = Number(evt.target.value);
		if (!num || num === 0) {
			customOption = '';
			return;
		}
		// @ts-ignore
		customOption = evt.target.value;
		// @ts-ignore
		chosenValue = Number(evt.target.value);
		chosenOption = -1;
	}
</script>

<div class="flex flex-col justify-center items-center">
	<div class="flex rounded-2xl h-10 gap-0.5 overflow-hidden relative">
		<div
			class="absolute -translate-x-1/2 transition-all bottom-0 h-[3px] w-10 bg-primary-light dark:bg-primary-dark rounded-t-full"
			style={`left: ${
				usingCustomOption
					? options.length * 5 + 2.5 + 'rem'
					: options.findIndex((e) => e.value === chosenOption) * 5.125 + 2.5 + 'rem'
			}`}
		/>
		{#each options as option}
			<button
				on:click={() => {
					chosenOption = option.value;
					chosenValue = option.value;
					usingCustomOption = false;
				}}
				class={`${
					chosenOption === option.value ? 'text-primary-light dark:text-primary-dark' : ''
				} w-20 h-full relative transition-colors flex flex-col items-center justify-center`}
			>
				<span class={`${option?.subText ? 'text-sm' : ''}`}>{option.text}</span>
				{#if option?.subText}
					<span class="text-xs">{option.subText}</span>
				{/if}
			</button>
		{/each}

		{#if usingCustomOption}
			<div
				class={`${
					usingCustomOption ? 'text-primary-light dark:text-primary-dark' : ''
				} w-20 h-full flex items-center justify-center gap-[1px] relative`}
			>
				<!-- svelte-ignore a11y-autofocus -->
				<input
					class="bg-transparent w-full h-full outline-none"
					style={customValueAnnotation ? 'text-align: right;' : 'text-align: center;'}
					type="string"
					maxlength="4"
					autofocus
					bind:value={customOption}
					on:input={onCustomInput}
				/>
				{#if customValueAnnotation}
					<span class="mr-4 pt-0.5">{customValueAnnotation}</span>
				{/if}
			</div>
		{:else}
			<button
				on:click={() => {
					usingCustomOption = true;
					chosenOption = -1;
				}}
				class={`bg-containers-0-light dark:bg-containers-0-dark w-20 h-full`}>Custom</button
			>
		{/if}
	</div>
</div>
