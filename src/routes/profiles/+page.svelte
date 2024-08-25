<script lang="ts">
	import ActionButton from '$lib/components/ActionButton.svelte';
	import Icon from '@iconify/svelte';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';

	export let data: PageData;

	let profiles = data.profiles;

	const handleCreate = async () => await goto('profiles/create');
</script>

<main class="flex flex-grow flex-col pl-2 pt-5 font-sans">
	{#each profiles as { name, id } (id)}
		<a
			href="/profiles/edit?id={id}"
			class="profile">
			{name}
		</a>
	{:else}
		<p class="ml-2 self-center text-left text-lg">
			No profiles found D:
			<br />
			Checkout the button in bottom right corner to create new profile
		</p>
	{/each}
</main>

<ActionButton on:click={handleCreate}>
	<Icon
		icon="line-md:plus"
		width="24"
		height="24" />
</ActionButton>

<style lang="scss">
	.profile {
		margin: 0.5rem;
		background-color: theme('colors.pallete.bg');
		padding: {
			top: 0.5rem;
			bottom: 0.5rem;
			left: 1.25rem;
			right: 1.25rem;
		}
		border-radius: 0.25rem;

		&::first-letter {
			--at-apply: bg-gradient-from-pallete-accent to-pallete-complement bg-clip-text text-xl
				font-bold bg-gradient-linear text-transparent;
		}
	}
</style>
