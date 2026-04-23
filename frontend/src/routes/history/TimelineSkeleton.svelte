<script lang="ts">
	let { count = 6 }: { count?: number } = $props();
	const indices = $derived(Array.from({ length: count }, (_, i) => i));
</script>

{#each indices as i (i)}
	<div class="flex gap-2 sm:gap-4">
		<div class="flex shrink-0 flex-col items-center">
			<div class="h-2 w-2 rounded-full bg-divider sm:h-3 sm:w-3"></div>
			<div class="h-full min-h-20 w-0.5 bg-divider"></div>
		</div>
		<div class="min-w-0 flex-1 pb-8">
			<div class="rounded-lg bg-surface p-3 sm:p-4">
				<div class="mb-3 flex items-center justify-between gap-2">
					<div class="h-3 w-24 animate-pulse rounded bg-background-highlight sm:w-32"></div>
					<div class="h-3 w-16 animate-pulse rounded bg-background-highlight"></div>
				</div>
				<div class="flex gap-2 sm:gap-3">
					<div class="h-16 w-16 shrink-0 animate-pulse rounded-md bg-background-highlight sm:h-20 sm:w-20"></div>
					<div class="flex flex-1 flex-col gap-2 pt-1">
						<div class="h-4 w-3/4 animate-pulse rounded bg-background-highlight"></div>
						<div class="h-3 w-1/2 animate-pulse rounded bg-background-highlight"></div>
					</div>
				</div>
			</div>
		</div>
	</div>
{/each}
