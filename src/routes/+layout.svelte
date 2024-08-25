<script
	lang="ts"
	context="module">
	const { getCurrent } = await import('@tauri-apps/api/window');
	await getCurrent().setDecorations(true);
</script>

<script lang="ts">
	import Navigation from '$lib/components/Navigation.svelte';
	import ProfileSelector from '$lib/components/ProfileSelector.svelte';
	import Toster from '$lib/components/Toster.svelte';
	import '../app.scss';
	import type { LayoutData } from './$types';

	export let data: LayoutData;
</script>

<Toster>
	<div class="waved grid grid-cols-12 grid-rows-10 h-screen w-screen text-pallete-text">
		<div class="col-start-4 col-end-13 w-full flex flex-col">
			<div class="h-24 w-full flex flex-row items-center justify-between align-middle">
				<h1 class="cursor-default select-none self-center golden-text px-5 text-center text-4xl">
					Golden Order
				</h1>
				<ProfileSelector {...data} />
			</div>
		</div>
		<Navigation />
		<div class="col-span-12 row-span-10 col-start-4 row-start-2 h-full w-full">
			<slot />
		</div>
	</div>
</Toster>

<style lang="scss">
	.waved {
		background-image: url('../lib/assets/layered-waves-haikei.svg');
	}
</style>
