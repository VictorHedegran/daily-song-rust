<script lang="ts">
	import SearchInput from './SearchInput.svelte';

	let { children } = $props();
</script>

<div class="flex flex-col gap-4">
	<div
		class="sticky top-(--header-height) z-10 border-b border-divider bg-background/95 px-4 pt-3 pb-3 backdrop-blur-sm"
	>
		<SearchInput />
	</div>
	{@render children()}
</div>
