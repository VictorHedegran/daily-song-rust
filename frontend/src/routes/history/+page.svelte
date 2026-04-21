<script lang="ts">
	import TimelineItem from './TimelineItem.svelte';
	import { Clock } from 'lucide-svelte';

	let { data } = $props();
	$effect(() => {
		console.log(data);
	});

</script>

{#if !data.success}
	<div class="py-16 text-center">
		<p class="text-lg font-medium text-text-secondary">Failed to load history</p>
		<p class="mt-2 text-sm text-text-secondary">Please try again later.</p>
	</div>
{:else}
	<div class="py-4 sm:py-6">
		<div class="mb-4 sm:mb-6">
			<div class="mb-2 flex items-center gap-2 sm:gap-3">
				<Clock class="h-6 w-6 text-spotify-green sm:h-8 sm:w-8" />
				<h1 class="text-2xl font-bold text-white sm:text-3xl">History</h1>
			</div>
			<p class="text-sm text-text-secondary sm:text-base">
				Our musical journey — all the songs we&apos;ve added
			</p>
		</div>

		{#if !data.result.length}
			<div class="py-16 text-center">
				<Clock class="mx-auto mb-4 h-12 w-12 text-text-secondary sm:h-16 sm:w-16" />
				<p class="text-lg font-medium text-text-secondary">No submissions yet</p>
				<p class="mt-2 text-sm text-text-secondary">Start by adding your first song!</p>
			</div>
		{:else}
			<div class="mt-6 sm:mt-8">
				{#each data.result as track, index (track.spotify_details?.id ?? index)}
					<TimelineItem {track} />
				{/each}
			</div>
		{/if}
	</div>
{/if}
