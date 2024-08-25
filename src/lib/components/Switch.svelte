<script lang="ts">
	import { createSwitch, createSync, melt } from '@melt-ui/svelte';
	import { spring } from 'svelte/motion';

	let buttonWidth = 2.75;
	let buttonPadding = 0.125;
	let thumbSize = 1.25;

	let thumbPosition = spring(buttonPadding, {
		damping: 0.265,
		precision: 0.09,
		stiffness: 0.2
	});

	export let disable = false;
	export let switched = false;
	export let onSwitchChange: ((value: boolean) => Promise<boolean>) | undefined = undefined;
	$: prevState = switched;

	const {
		elements: { root: switchRoot, input },
		states: { checked },
		options: { disabled, defaultChecked }
	} = createSwitch({});

	$: if (prevState !== $checked && onSwitchChange) {
		onSwitchChange($checked).then((val) => {
			const old = $checked;
			checked.update(
				() => val,
				() => (prevState = old)
			);
		});
	}
	$: $checked = switched;

	const sync = createSync({ disabled, checked, defaultChecked });
	$: sync.disabled(disable, (v) => (disable = v));
	$: thumbPosition.set($checked ? buttonWidth - thumbSize - buttonPadding : buttonPadding);
</script>

<button
	class="relative h-6 cursor-default rounded-full bg-pallete-accent transition-colors data-[disabled=true]:bg-neutral-500 data-[state=checked]:bg-green-600"
	style:width="{buttonWidth}rem"
	use:melt={$switchRoot}>
	<span
		class="block rounded-full bg-white"
		style:width="{thumbSize}rem"
		style:height="{thumbSize}rem"
		style="transform: translateX({$thumbPosition}rem);" />
</button>
<input use:melt={$input} />
