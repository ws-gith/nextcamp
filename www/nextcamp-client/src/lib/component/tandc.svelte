<script lang="ts">
	import { PUBLIC_DEV_USERNAME } from '$env/static/public';
	import { slide } from 'svelte/transition';
	let tAndC = $state(true);
</script>

<button class="text-xs underline" onclick={(_) => (tAndC = !tAndC)}>
	{#if tAndC}
		Hide
	{:else}
		Show
	{/if} Terms & Conditions
</button>

{#if tAndC}
	<div class="tc w-fit text-white" transition:slide={{ duration: 200 }}>
		<ul class="list-inside list-disc space-y-2 text-left">
			<li>This tool is provided 'as-is' for educational or idea validation purposes only.</li>
			<li>
				The creator <a href="/" class="font-bold underline">{PUBLIC_DEV_USERNAME}</a> disclaims all liability
				for any misuse or illegal activity.
			</li>
			<li>You are solely responsible for any abuse or misconduct in using this tool.</li>
			<li>Reselling or purchasing this tool is strictly prohibited.</li>
			<li>
				For any inquiries or issues, please contact the creator
				<a href="/" class="font-bold underline">{PUBLIC_DEV_USERNAME}</a>.
			</li>
		</ul>
		<p class="mt-2">By using this tool, you agree to these terms.</p>
	</div>
{/if}

<style lang="postcss">
	.tc {
		@apply rounded bg-green-900 p-3 text-center text-xs leading-tight transition-transform duration-200;
	}
</style>
