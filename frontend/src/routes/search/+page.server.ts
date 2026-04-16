import { TracksPageSchema, type TracksPage } from '$lib/types/schemas';
import type { PageServerLoad, Actions } from './$types';
import { error } from '@sveltejs/kit';

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

export const actions: Actions = {
	addTrack: async ({ request, fetch }) => {
		const data = await request.formData();
		const uri = data.get('uri');
		const notes = data.get('notes');
		const mood = data.get('mood');

		if (!uri) throw error(400, 'Missing uri');

		const res = await fetch('http://127.0.0.1:3000/tracks', {
			method: 'POST',
			credentials: 'include',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ uri, notes, mood })
		});

		if (!res.ok) throw error(res.status, 'Failed to add track');
	}
};
