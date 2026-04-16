<script lang="ts">
	import { LogOut } from 'lucide-svelte';
</script>

<button
	onclick={() => {}}
	class="flex h-8 w-8 items-center justify-center rounded-full text-text-subdued transition-all duration-150 hover:bg-white/10 hover:text-white active:scale-95 disabled:opacity-40"
	aria-label="Logout"
	title="Logout"
>
	<LogOut size={15} />
</button>
