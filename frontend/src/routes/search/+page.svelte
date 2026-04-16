<script lang="ts">
	import type { PageProps } from './$types';
	import SearchResultCard from './SearchResultCard.svelte';

	let { data }: PageProps = $props();
</script>

<div class="flex flex-col gap-1.5">
	{#each data.result.items as track (track.id)}
		<SearchResultCard {track} />
	{/each}
</div>
