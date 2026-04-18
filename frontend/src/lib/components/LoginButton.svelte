<script lang="ts">
	import { LogIn } from 'lucide-svelte';
</script>

<a
	href="http://127.0.0.1:3000/auth/spotify"
	class="flex h-8 w-8 items-center justify-center rounded-full text-text-subdued transition-all duration-150 hover:bg-white/10 hover:text-white active:scale-95"
	aria-label="Login"
	title="Login"
>
	<LogIn size={15} />
</a>
