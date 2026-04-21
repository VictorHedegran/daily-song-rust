<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import BottomNav from '$lib/components/BottomNav.svelte';
	import Header from '$lib/components/Header.svelte';

	let { children, data } = $props();

	let user = $derived(data.success ? data.result : null);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<Header {user} />
<main class="max-w-4xl mx-auto px-4">{@render children()}</main>
<BottomNav />
