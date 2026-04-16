import type { TracksPage } from '$lib/types/TracksPage';
import type { PageServerLoad } from './$types';

const emptyPage: TracksPage = { items: [], total: 0, limit: 0, offset: 0, href: '', next: null, previous: null };

export const load: PageServerLoad = async ({ url, fetch }) => {
	const query = url.searchParams.get('q')?.trim() ?? '';
	if (!query) return { query, results: emptyPage };

	const res = await fetch(`http://127.0.0.1:3000/search?query=${encodeURIComponent(query)}`, {
		credentials: 'include'
	});
	if (!res.ok) return { query, results: emptyPage, error: res.statusText };

	const results: TracksPage = await res.json();
	return { query, results };
};
