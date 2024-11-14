<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	let name = $state('');
	let greetMsg = $state('');

	async function greet(event: Event) {
		event.preventDefault();
		greetMsg = await invoke('greet', { name });
	}
</script>

<div class="center p-8 flex-col gap-2">
	<h1>Hey You in tauri land!</h1>

	{#if greetMsg}
		<p>{greetMsg}</p>
	{/if}

	<form onsubmit={greet}>
		<input type="text" class="h-10 p-2 border border-gray-900" bind:value={name} />

		<button type="submit">Greet</button>
	</form>
</div>
