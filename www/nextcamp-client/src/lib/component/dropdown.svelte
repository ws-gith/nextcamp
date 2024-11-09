<script lang="ts">
	import { Dropdown } from '$icon';

	const {
		selector,
		label,
		card_class = 'border-red-900'
	}: { selector: string; label: string; card_class?: string } = $props();
</script>

<ul class={'w-60 border-4 lg:w-96 ' + card_class}>
	<button class="between size-full p-2 pt-6 text-left text-sm">
		<p class="flex flex-col">
			<span class="text-[10px]">{selector}</span>
			<span>{label}</span>
		</p>

		<Dropdown width={22} height={22} />
	</button>
</ul>
