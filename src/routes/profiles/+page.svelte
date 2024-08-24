<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;

	let profiles = data.profiles;
</script>

<main class="flex flex-grow flex-col pl-2 pt-5">
	{#each profiles as { name, id } (id)}
		<a
			href="/profiles/{id}"
			class="first-letter:to-royal-indigo-300 m-2 rounded bg-zinc-700 px-5 py-2 first-letter:from-fuchsia-400 first-letter:bg-gradient-to-b first-letter:bg-clip-text first-letter:text-xl first-letter:text-transparent">
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

<a
	href="profiles/create"
	class="to-royal-indigo-500 absolute bottom-2 right-2 m-0 rounded-full from-cyan-600 bg-gradient-to-r px-4 py-2.5 shadow">
	+
	<!-- TODO: Add + Icon and add tooltip -->
</a>
