<script lang="ts">
	import { House, Search, Clock } from 'lucide-svelte';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';

	const navItems = [
		{ icon: House, label: 'Home', href: '/' as const },
		{ icon: Search, label: 'Search', href: '/search' as const },
		{ icon: Clock, label: 'History', href: '/history' as const }
	];
</script>

<nav
	class="fixed right-0 bottom-0 left-0 z-50 max-w-full border-t border-divider bg-background pb-[env(safe-area-inset-bottom)] will-change-transform"
>
	<div class="mx-auto flex max-w-4xl items-center justify-around px-2 py-1 sm:px-4">
		{#each navItems as item (item.href)}
			{@const isActive =
				page.url.pathname === item.href ||
				(item.href !== '/' && page.url.pathname.startsWith(item.href))}
			<a
				href={resolve(item.href)}
				class="flex min-h-12 min-w-12 flex-col items-center justify-center gap-0.5 rounded-lg px-4 py-2 transition-all duration-150 active:scale-95 active:bg-background-highlight sm:px-6"
			>
				<item.icon
					class="h-5 w-5 transition-colors sm:h-6 sm:w-6 {isActive
						? 'text-spotify-green'
						: 'text-text-secondary'}"
				/>
				<span
					class="text-[11px] font-medium whitespace-nowrap transition-colors {isActive
						? 'text-white'
						: 'text-text-secondary'}"
				>
					{item.label}
				</span>
			</a>
		{/each}
	</div>
</nav>
