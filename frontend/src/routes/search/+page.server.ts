import type { Track } from '$lib/types/Track';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url, fetch }) => {
	const query = url.searchParams.get('q')?.trim() ?? '';
	if (!query) return { query, results: [] as Track[] };

	const res = await fetch(`http://127.0.0.1:3000/search?query=${encodeURIComponent(query)}`, {
		credentials: 'include'
	});
	if (!res.ok) return { query, results: [] as Track[], error: res.statusText };

	return { query, results: (await res.json()) as Track[] };
};
