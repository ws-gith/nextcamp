<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>

<div class="mx-auto size-full max-w-7xl space-y-10 px-8">
	{@render children()}
</div>

<style lang="postcss">
	:global(html) {
		@apply relative;
	}

	:global(body) {
		@apply min-h-screen bg-gradient-to-b from-red-950 from-10% to-green-950 bg-no-repeat font-sans text-yellow-400;
	}
</style>
