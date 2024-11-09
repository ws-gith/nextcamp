<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children, router, feed }: { router: Snippet; children: Snippet; feed: Snippet } = $props();
</script>

<div class="flex gap-2">
	<div class="space-y-3">
		{@render router()}
		{@render feed()}
	</div>

	{@render children()}
</div>
