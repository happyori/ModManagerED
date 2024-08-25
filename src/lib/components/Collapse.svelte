<script lang="ts">
	import { createCollapsible, melt } from '@melt-ui/svelte';
	import { cubicInOut } from 'svelte/easing';
	import { tweened } from 'svelte/motion';
	import { slide } from 'svelte/transition';

	export let title: string;

	let chevronRotation = tweened(180, {
		duration: 200,
		easing: cubicInOut
	});

	const {
		elements: { root, trigger, content },
		states: { open }
	} = createCollapsible({
		forceVisible: true
	});

	$: chevronRotation.set($open ? 0 : 180);
</script>

<div use:melt={$root}>
	<div class="{$$props.class} group flex justify-end">
		<button
			use:melt={$trigger}
			class="flex flex-row items-center justify-end border-b border-b-1 border-b-pallete-accent space-x-2"
			type="button">
			<span class="select-none text-accents-2 transition-colors group-hover:text-accents-1">
				{title}
			</span>
			<svg
				class="text-accents-2 transition-colors group-hover:text-accents-1"
				xmlns="http://www.w3.org/2000/svg"
				width="1em"
				height="1em"
				style="transform: rotate({$chevronRotation}deg);"
				viewBox="0 0 24 24">
				<path
					fill="none"
					stroke="currentColor"
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="m17 14l-5-5l-5 5" />
			</svg>
		</button>
	</div>
</div>

{#if $open}
	<section
		transition:slide
		use:melt={$content}>
		<slot />
	</section>
{/if}
