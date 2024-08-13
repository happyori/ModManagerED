<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;

	let profiles = data.profiles;
</script>

<main class="pt-5 pl-2 flex flex-grow flex-col">
	{#each profiles as { name, id } (id)}
		<a
			href="/profiles/{id}"
			class="bg-zinc-700 m-2 rounded px-5 py-2
					first-letter:bg-clip-text first-letter:bg-gradient-to-b first-letter:from-fuchsia-400
					first-letter:to-royal-indigo-300 first-letter:text-transparent first-letter:text-xl">
			{name}
		</a>
	{:else}
		<p class="text-lg ml-2 text-left self-center">
			No profiles found D:
			<br />
			Checkout the button in bottom right corner to create new profile
		</p>
	{/each}
</main>

<a
	href="profiles/create"
	class="absolute bottom-2 right-2 bg-gradient-to-r from-cyan-600 to-royal-indigo-500 rounded-full shadow px-4 py-2.5 m-0">
	+
	<!-- TODO: Add + Icon and add tooltip -->
</a>
