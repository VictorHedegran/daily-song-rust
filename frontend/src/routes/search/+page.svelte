<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

{#if !data.query}
	<p>Type to search.</p>
{:else if data.error}
	<p>Search failed: {data.error}</p>
{:else if data.results.items.length === 0}
	<p>No results for "{data.query}".</p>
{:else}
	<ul>
		<p>{JSON.stringify(data.results)} for "{data.query}".</p>
	</ul>
{/if}
