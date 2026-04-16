<script lang="ts">
	import { Music } from 'lucide-svelte';
	import HeaderUser from './HeaderUser.svelte';
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
		<HeaderUser />
	</div>
</header>
