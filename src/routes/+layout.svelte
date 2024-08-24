<script lang="ts">
	import ProfileSelector from '$lib/components/ProfileSelector.svelte';
	import Toster from '$lib/components/Toster.svelte';
	import '../app.css';
	import type { LayoutData } from './$types';

	export let data: LayoutData;
</script>

<Toster>
	<div class="grid grid-cols-12 grid-rows-10 h-screen w-screen bg-zinc-900 text-white">
		<div
			id="action_corner"
			class="z-10 col-span-3 row-span-3 flex flex-col items-center justify-center rounded-br-3xl bg-zinc-700 drop-shadow-xl">
			<enhanced:img
				src="$lib/assets/eldenringlogo.png?w=100"
				class="mb-4"
				alt="Golden Order Logo" />
			<ProfileSelector {...data} />
		</div>
		<div class="col-start-4 col-end-13 w-full flex flex-col">
			<div class="h-24 w-full flex flex-row justify-between bg-zinc-800 align-middle">
				<h1
					class="cursor-default select-none self-center from-yellow-500 to-amber-600 to-60% bg-gradient-to-br bg-clip-text px-5 text-center text-4xl text-transparent font-bold">
					Golden Order
				</h1>
			</div>
		</div>

		<nav
			class="col-span-3 row-span-10 row-start-4 h-auto max-w-fit flex flex-col justify-between gap-2 bg-zinc-800 pr-20 pt-5">
			<div class="flex flex-col *:ml-2 *:mr-6">
				<a
					class="p-2 text-white"
					href="/profiles">
					Profiles
				</a>
				<a
					class="p-2 text-white"
					href="/mods">
					Mods
				</a>
			</div>
			<a
				href="/settings"
				class="ml-2 mr-4 p-2 text-white">
				Settings
			</a>
		</nav>
		<div class="col-span-12 row-span-10 col-start-4 row-start-2 h-full w-full">
			<slot />
		</div>
	</div>
</Toster>
