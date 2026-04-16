<script lang="ts">
	import { LoaderCircle, Search, X } from 'lucide-svelte';
	import { page } from '$app/state';

	let query = $state(page.url.searchParams.get('q') ?? '');
	let isSearching = $state(false);
	let debounceTimer: ReturnType<typeof setTimeout>;

	function handleInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		query = value;
		isSearching = true;
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			isSearching = false;
		}, 500);
	}

	function clear() {
		query = '';
		isSearching = false;
	}
</script>

<div
	class="m-auto flex max-w-96 items-center rounded-full bg-input px-4 py-1 transition-colors duration-200 focus-within:ring-2 focus-within:ring-white/20"
>
	{#if isSearching && query}
		<LoaderCircle class="h-5 w-5 shrink-0 animate-spin text-text-secondary" aria-hidden="true" />
	{:else}
		<Search class="h-5 w-5 shrink-0 text-text-secondary" aria-hidden="true" />
	{/if}
	<label for="search-input" class="sr-only">Search for a song</label>
	<form action="/search" class=" flex-1 items-center">
		<input
			id="search-input"
			name="q"
			value={query}
			oninput={handleInput}
			placeholder="Search for a song..."
			class="flex-1 border-none bg-transparent text-white placeholder:text-text-subdued focus:ring-0 focus:outline-none"
		/>
	</form>
	{#if query}
		<button
			onclick={clear}
			class="shrink-0 p-1 text-text-subdued transition-colors active:text-white"
			aria-label="Clear search"
		>
			<X class="h-4 w-4" />
		</button>
	{/if}
</div>
