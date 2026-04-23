<script lang="ts">
	import { LoaderCircle, Search, X } from 'lucide-svelte';
	import { page, navigating } from '$app/state';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let query = $state(page.url.searchParams.get('q') ?? '');
	let debounceTimer: ReturnType<typeof setTimeout>;

	function handleInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		query = value;
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			goto(resolve(`/search?q=${encodeURIComponent(value)}`), { replaceState: true, keepFocus: true, noScroll: true });
		}, 300);
	}

	function clear() {
		query = '';
		clearTimeout(debounceTimer);
		goto(resolve('/search?q='), { replaceState: true, keepFocus: true, noScroll: true });
	}
</script>

<div
	class="m-auto flex max-w-96 items-center rounded-full bg-input px-4 py-1 transition-colors duration-200 focus-within:ring-2 focus-within:ring-white/20"
>
	{#if navigating.type !== null && query}
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
