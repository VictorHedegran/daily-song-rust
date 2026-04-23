<script lang="ts">
	import { ExternalLink, Music, ChevronDown, ChevronUp } from 'lucide-svelte';
	import type { GetHistoryResponse } from '$lib/types/schemas';
	//import Comments from './Comments';

	let isExpanded = $state(false);
	//let showComments = $state(false);

	let { track }: { track: GetHistoryResponse } = $props();
</script>

<div class="flex gap-2 sm:gap-4">
	<div class="flex shrink-0 flex-col items-center">
		<div class="h-2 w-2 rounded-full bg-spotify-green sm:h-3 sm:w-3"></div>
		<div class="h-full min-h-20 w-0.5 bg-divider"></div>
	</div>

	<div class="min-w-0 flex-1 pb-8">
		<div
			class="rounded-lg bg-surface p-3 transition-colors duration-150 active:bg-background-highlight sm:p-4 sm:hover:bg-background-highlight"
		>
			<div class="mb-3 flex min-w-0 items-center justify-between gap-2">
				<div class="flex min-w-0 flex-1 items-center gap-2">
					<span class="truncate text-xs text-text-secondary sm:text-sm">
						{track.spotify_details?.artists[0]?.name || 'Unknown Artist'}
					</span>
				</div>
				<time class="shrink-0 text-xs text-text-subdued sm:text-sm">
					{track.date}
				</time>
			</div>

			<div class="flex min-w-0 gap-2 sm:gap-3">
				{#if track.spotify_details?.album?.images[0]?.url}
					<img
						src={track.spotify_details.album.images[0].url}
						alt={track.spotify_details.name || 'Album art'}
						width={64}
						height={64}
						sizes="(min-width: 640px) 80px, 64px"
						loading="lazy"
						decoding="async"
						class="h-16 w-16 shrink-0 rounded-md sm:h-20 sm:w-20"
					/>
				{:else}
					<div
						class="flex h-16 w-16 shrink-0 items-center justify-center rounded-md bg-background-highlight sm:h-20 sm:w-20"
					>
						<Music size={24} class="text-text-subdued sm:h-8 sm:w-8" />
					</div>
				{/if}
				<div class="min-w-0 flex-1">
					<h3 class="truncate text-sm font-semibold text-white sm:text-base">
						{track.spotify_details?.name || 'Unknown Track'}
					</h3>
					<p class="truncate text-xs text-text-secondary sm:text-sm">
						{track.spotify_details?.artists.map((a) => a.name).join(', ') || 'Unknown Artist'}
					</p>

					{#if track.mood}
						<div class="mt-2">
							<span
								class="inline-block rounded-full bg-background-highlight px-2 py-0.5 text-xs font-medium text-spotify-green sm:px-3 sm:py-1"
							>
								{track.mood}
							</span>
						</div>
					{/if}

					{#if track.notes}
						<div class="mt-2">
							<p
								class={`text-xs text-text-secondary sm:text-sm ${isExpanded ? '' : 'line-clamp-3'}`}
							>
								{track.notes}
							</p>
							{#if track.notes.split('\n').length > 3 || track.notes.length > 150}
								<button
									onclick={() => (isExpanded = !isExpanded)}
									class="mt-1 flex items-center gap-1 py-1.5 text-xs text-spotify-green transition-colors active:text-spotify-green-hover"
									aria-expanded={isExpanded}
								>
									<span>{isExpanded ? 'Show less' : 'Show more'}</span>
									{#if isExpanded}
										<ChevronUp size={12} />
									{/if}
									{#if !isExpanded}
										<ChevronDown size={12} />
									{/if}
								</button>
							{/if}
						</div>
					{/if}

					{#if track.spotify_details?.external_urls?.spotify}
						<a
							href={track.spotify_details.external_urls.spotify}
							target="_blank"
							rel="noopener noreferrer external"
							class="mt-2 inline-flex items-center gap-1.5 py-1.5 text-xs text-spotify-green transition-colors active:text-spotify-green-hover"
						>
							<span>Open in Spotify</span>
							<ExternalLink size={12} />
						</a>
					{/if}
				</div>
			</div>
			<!-- 
			<button
				onclick={() => (showComments = !showComments)}
				class="mt-3 flex items-center gap-1.5 py-2 text-xs text-text-subdued transition-colors active:text-spotify-green"
				aria-expanded={showComments}
			>
				<MessageCircle size={13} />
				<span>
					{showComments
						? 'Hide comments'
						: track.commentCount > 0
							? `${track.commentCount} comment${track.commentCount === 1 ? '' : 's'}`
							: 'Comments'}
				</span>
				{#if showComments}
					<ChevronUp size={12} />
				{/if}
				{#if !showComments}
					<ChevronDown size={12} />
				{/if}
			</button>

			{#if showComments}
				<Comments submissionId={track.id} />
			{/if}
        -->
		</div>
	</div>
</div>
