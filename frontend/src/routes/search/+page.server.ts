import { TracksPageSchema, type TracksPage } from '$lib/types/schemas';
import type { PageServerLoad } from './$types';

const emptyPage: TracksPage = {
	items: [],
	total: 0,
	limit: 0,
	offset: 0,
	href: '',
	next: null,
	previous: null
};

export const load: PageServerLoad = async ({
	url,
	fetch
}): Promise<{ query: string; result: TracksPage; error?: string }> => {
	const query = url.searchParams.get('q')?.trim() ?? '';
	if (!query) return { query, result: emptyPage };

	const res = await fetch(`http://127.0.0.1:3000/search?query=${encodeURIComponent(query)}`, {
		credentials: 'include'
	});
	if (!res.ok) return { query, result: emptyPage, error: res.statusText };

	const result = TracksPageSchema.safeParse(await res.json());
	if (!result.success) return { query, result: emptyPage, error: 'Failed to parse tracks page' };

	return { query, result: result.data };
};
