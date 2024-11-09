<script>
	import { Dropdown, TandC, User, Wrapper } from '$component';
</script>

<header class="relative space-y-4">
	<p class="text-2xl italic">X~CAMP</p>
	<User />
	<TandC />

	<div class="center absolute bottom-4 right-4 flex-col gap-2">
		<span class="text-[9px] italic">Version: v0.1.0</span>
		<span class="text-3xl">💥</span>
	</div>
</header>

{#snippet router()}
	<Dropdown selector="Select view:" label="Manage Smtp" />
{/snippet}

<h2>Manage View</h2>
<Wrapper {router}>
	{#snippet feed()}
		<div class="router-feed-skin"></div>
	{/snippet}

	<div class="router-main-skin"></div>
</Wrapper>

<!-- ##############################->Task Management<-##################################### -->

{#snippet task_router()}
	<Dropdown
		selector="Select campaign type:"
		label="Email - SMTP(beta-v0.1.0)"
		card_class="border-green-900 text-green-400"
	/>
{/snippet}

<h2 class="text-green-900">Services</h2>
<Wrapper router={task_router}>
	{#snippet feed()}
		<div class="router-task-feed-skin"></div>
	{/snippet}

	<div class="router-task-main-skin"></div>
</Wrapper>

<style lang="postcss">
	h2 {
		@apply text-2xl uppercase;
	}

	header {
		@apply sticky top-0 z-10 bg-gradient-to-b from-green-950 to-red-950 bg-no-repeat p-8;
	}
	.router-feed-skin,
	.router-main-skin,
	.router-task-feed-skin,
	.router-task-main-skin {
		@apply border-4;
	}

	.router-task-main-skin,
	.router-main-skin {
		@apply flex-1;
	}
	.router-feed-skin {
		@apply h-32 border-yellow-400;
	}

	.router-main-skin {
		@apply border-green-900;
	}

	.router-task-feed-skin {
		@apply h-32 border-red-900;
	}

	.router-task-main-skin {
		@apply border-yellow-400;
	}
</style>
