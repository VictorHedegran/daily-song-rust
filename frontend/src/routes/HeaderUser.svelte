<script lang="ts">
	import LogoutButton from './LogoutButton.svelte';
</script>

<div class="flex items-center gap-2">
	<div
		class="flex items-center gap-2.5 rounded-full bg-background-highlight/70 py-1 pr-1 pl-3 transition-colors hover:bg-background-highlight"
	>
		<div class="flex h-7 w-7 items-center justify-center rounded-full bg-spotify-green/20">
			<span class="text-xs font-semibold text-spotify-green">
				{'name'.charAt(0).toUpperCase()}
			</span>
		</div>
		<span class="hidden max-w-25 truncate text-sm font-medium text-text-secondary sm:block">
			name
		</span>
		<LogoutButton />
	</div>
</div>
