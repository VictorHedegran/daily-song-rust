<script lang="ts">
	import { LogIn } from 'lucide-svelte';
	import { PUBLIC_BACKEND_URL } from '$env/static/public';
</script>

<a
	href="{PUBLIC_BACKEND_URL}/auth/spotify"
	rel="external"
	class="flex h-8 w-8 items-center justify-center rounded-full text-text-subdued transition-all duration-150 hover:bg-white/10 hover:text-white active:scale-95"
	aria-label="Login"
	title="Login"
>
	<LogIn size={15} />
</a>
