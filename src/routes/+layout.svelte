<script lang="ts">
	import ProfileSelector from '$lib/components/ProfileSelector.svelte';
	import Toster from '$lib/components/Toster.svelte';
	import '../app.css';
	import type { LayoutData } from './$types';

	export let data: LayoutData;
</script>

<Toster>
	<div class="grid grid-cols-12 grid-rows-10 w-screen h-screen bg-zinc-900 text-white">
		<div
			id="action_corner"
			class="col-span-3 row-span-3 flex z-10 rounded-br-3xl drop-shadow-xl justify-center items-center flex-col bg-zinc-700">
			<enhanced:img
				src="$lib/assets/eldenringlogo.png?w=100"
				class="mb-4"
				alt="Golden Order Logo" />
			<ProfileSelector {...data} />
		</div>
		<div class="col-start-4 col-end-13 flex flex-col w-full">
			<div class="flex flex-row justify-between w-full h-24 align-middle bg-zinc-800">
				<h1
					class="bg-clip-text bg-gradient-to-br text-4xl text-transparent font-bold from-yellow-500 px-5 cursor-default select-none text-center self-center to-amber-600 to-60%">
					Golden Order
				</h1>
			</div>
		</div>

		<nav
			class="row-span-10 row-start-4 col-span-3 max-w-fit h-auto bg-zinc-800 gap-2 pt-5 pr-20 flex flex-col justify-between">
			<div class="*:ml-2 *:mr-6 flex flex-col">
				<a
					class="text-white p-2"
					href="/profiles">
					Profiles
				</a>
				<a
					class="text-white p-2"
					href="/mods">
					Mods
				</a>
			</div>
			<a
				href="/settings"
				class="text-white p-2 ml-2 mr-4">
				Settings
			</a>
		</nav>
		<div class="row-start-2 row-span-10 col-start-4 col-span-12 w-full h-full">
			<slot />
		</div>
	</div>
</Toster>
