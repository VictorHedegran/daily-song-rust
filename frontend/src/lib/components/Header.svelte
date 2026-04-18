<script lang="ts">
	import { Music } from 'lucide-svelte';
	import LogoutButton from './LogoutButton.svelte';
	import type { UserResponse } from '$lib/types/schemas';
	import LoginButton from './LoginButton.svelte';

	let { user }: { user: UserResponse | null } = $props();
</script>

<header
	class="sticky top-0 z-10 h-(--header-height) w-full border-b border-divider/60 bg-linear-to-b from-background-elevated/98 to-background/95 backdrop-blur-md"
>
	<div class="mx-auto flex h-full max-w-4xl items-center justify-between px-4">
		<div class="flex items-center gap-2.5">
			<div class="flex h-8 w-8 items-center justify-center rounded-lg bg-spotify-green/15">
				<Music class="h-4 w-4 text-spotify-green" />
			</div>
			<div class="flex flex-col">
				<h1 class="text-lg leading-tight font-bold tracking-tight text-white md:text-xl">
					Our Daily Songs
				</h1>
				<span
					class="text-[10px] leading-tight font-medium tracking-widest text-text-subdued uppercase md:text-[11px]"
				>
					Collaborative Playlist
				</span>
			</div>
		</div>
		<div class="flex items-center gap-2">
			<div
				class="flex items-center gap-2.5 rounded-full bg-background-highlight/70 py-1 pr-1 pl-3 transition-colors hover:bg-background-highlight"
			>
				{#if user?.avatar_url}
					<img
						src={user.avatar_url}
						alt={user.name}
						width={28}
						height={28}
						class="h-7 w-7 rounded-full object-cover ring-1 ring-white/10"
					/>
				{:else if user}
					<div class="flex h-7 w-7 items-center justify-center rounded-full bg-spotify-green/20">
						<span class="text-xs font-semibold text-spotify-green">
							{user.name.charAt(0).toUpperCase()}
						</span>
					</div>
				{/if}
				<span class="hidden max-w-25 truncate text-sm font-medium text-text-secondary sm:block">
					{user ? user.name : 'Login'}
				</span>
				{#if user}
					<LogoutButton />
				{:else}
					<LoginButton />
				{/if}
			</div>
		</div>
	</div>
</header>
