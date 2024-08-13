<script
	lang="ts"
	context="module">
	export type ToastData = {
		title: string;
		content: string;
		type: ToastType;
	};
	export const ToasterContext = 'toaster';
	export type ToasterContextReturn = {
		addToast: ReturnType<typeof createToaster<ToastData>>['helpers']['addToast'];
	};
	type ToastType = 'error' | 'info' | 'warning';
	const getColor = (type: ToastType) => {
		switch (type) {
			default:
			case 'error':
				return 'bg-red-500';
			case 'info':
				return 'bg-blue-500';
			case 'warning':
				return 'bg-orange-500';
		}
	};
</script>

<script lang="ts">
	import { createToaster, melt } from '@melt-ui/svelte';
	import { setContext } from 'svelte';
	import { flip } from 'svelte/animate';
	import { fly } from 'svelte/transition';

	const {
		elements: { content, title, description, close },
		actions: { portal },
		helpers: { addToast },
		states: { toasts }
	} = createToaster<ToastData>();

	setContext(ToasterContext, {
		addToast
	});
</script>

<div
	use:portal
	class="fixed right-0 top-0 z-50 m-4 flex flex-col items-end gap-2 md:bottom-0 md:top-auto">
	{#each $toasts as { id, data } (id)}
		<div
			use:melt={$content(id)}
			animate:flip={{ duration: 200 }}
			in:fly={{ duration: 200, x: '100%' }}
			out:fly={{ duration: 200, x: '100%' }}
			class="rounded-lg bg-neutral-800 text-white shadow-md">
			<div
				class="relative flex w-[24rem] max-w-[calc(100vw-2rem)] items-center justify-between gap-4 p-5">
				<section>
					<h3
						use:melt={$title(id)}
						class="flex items-center gap-2 text-md font-bold tracking-tight">
						<span class="size-1.5 rounded-full inline-block {getColor(data.type)}" />
						{data.title}
					</h3>
					<div use:melt={$description(id)}>{data.content}</div>
				</section>
				<button
					use:melt={$close(id)}
					class="absolute right-4 top-4 grid size-6 place-items-center rounded-full text-royal-indigo-300 hover:bg-royal-indigo-600/50">
					<svg
						class="size-4"
						xmlns="http://www.w3.org/2000/svg"
						width="1em"
						height="1em"
						viewBox="0 0 24 24">
						<path
							fill="none"
							stroke="currentColor"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 12L7 7m5 5l5 5m-5-5l5-5m-5 5l-5 5" />
					</svg>
				</button>
			</div>
		</div>
	{/each}
</div>
<slot />
