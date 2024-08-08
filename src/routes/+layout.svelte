<script>
	import '../app.css';
</script>

<div class="flex flex-col justify-start w-screen h-screen bg-zinc-900 text-white">
	<div class="flex flex-row w-full">
		<div
			id="action_corner"
			class="flex z-10 p-4 px-8 rounded-br-3xl drop-shadow-xl w-auto min-w-56 justify-center items-center flex-col min-h-52 bg-zinc-700">
			<enhanced:img
				src="$lib/assets/eldenringlogo.png?w=100"
				class="mb-4"
				alt="Golden Order Logo" />
			<button class="bg-royal-indigo-700 p-2 rounded-full px-4 cursor-auto text-white">
				Click
			</button>
		</div>
		<div class="flex flex-col w-full">
			<div class="flex flex-row justify-between w-full h-24 align-middle bg-zinc-800">
				<h1
					class="bg-clip-text bg-gradient-to-br text-4xl text-transparent font-bold from-yellow-500 px-5 cursor-default select-none text-center self-center to-amber-600 to-60%">
					Golden Order
				</h1>
			</div>
		</div>
	</div>

	<div class="flex flex-row w-full h-full">
		<nav class="-mt-5 h-auto bg-zinc-800 min-w-52 gap-2 pt-5 flex flex-col">
			<a
				class="text-white p-2"
				href="/profiles">
				Profiles
			</a>
			<a
				class="text-white p-2"
				href="/mods">
				Main Mods
			</a>
			<a
				class="text-white p-2"
				href="/dlls">
				DLL Mods
			</a>
		</nav>
		<slot />
	</div>
</div>
