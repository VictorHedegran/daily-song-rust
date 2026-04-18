<script lang="ts">
	import { enhance } from '$app/forms';
	import { LogOut } from 'lucide-svelte';
</script>

<form method="POST" action="/?/logout" use:enhance>
	<button
		type="submit"
		class="flex h-8 w-8 items-center justify-center rounded-full text-text-subdued transition-all duration-150 hover:bg-white/10 hover:text-white active:scale-95 disabled:opacity-40"
		aria-label="Logout"
		title="Logout"
	>
		<LogOut size={15} />
	</button>
</form>
