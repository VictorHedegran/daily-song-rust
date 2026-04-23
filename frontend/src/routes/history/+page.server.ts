import { GetHistoryResponseSchema } from '$lib/types/schemas';
import * as z from 'zod';
import type { PageServerLoad } from './$types';
import { BACKEND_URL } from '$env/static/private';

export const load: PageServerLoad = ({ fetch }) => {
	const result = fetch(`${BACKEND_URL}/history`, { credentials: 'include' }).then(async (res) => {
		if (!res.ok) throw new Error(JSON.stringify(await res.json()));
		const parsed = z.array(GetHistoryResponseSchema).safeParse(await res.json());
		if (!parsed.success) throw new Error('Failed to parse history response');
		return parsed.data;
	});

	return { result };
};
