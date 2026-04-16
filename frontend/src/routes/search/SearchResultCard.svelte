<script lang="ts">
	import { type Track } from '$lib/types/schemas';
	import { Plus } from 'lucide-svelte';
	let { track }: { track: Track } = $props();

	const images = $derived(track.album.images);

	const thumbnail = $derived(
		images?.find((img) => img.width && img.width >= 64 && img.width <= 300) ||
			images?.[images.length - 1]
	);

	const artistNames = $derived(track.artists?.map((artist) => artist.name).join(', '));
</script>

<div
	class="group flex w-full max-w-2xl items-center gap-3 rounded-lg bg-surface p-3 transition-colors duration-150 active:bg-background-highlight sm:hover:bg-background-highlight"
>
	<img
		class="shrink-0 rounded-md"
		src={thumbnail?.url}
		alt={`${track.name} album art`}
		width={56}
		height={56}
		sizes="56px"
	/>
	<div class="min-w-0 flex-1">
		<h3 class="truncate text-[15px] font-semibold text-white" title={track.name}>
			{track.name}
		</h3>
		<p class="truncate text-sm text-text-secondary" title={artistNames}>
			{artistNames}
		</p>
	</div>
	<div class="shrink-0">
		<button
			class="rounded-full p-2.5 text-sm font-bold transition-all duration-200 active:scale-90"
		>
			<Plus class="h-5 w-5" aria-hidden="true" />
		</button>
	</div>
</div>
