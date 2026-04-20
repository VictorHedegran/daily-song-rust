import { type GetHistoryResponse, GetHistoryResponseSchema } from '$lib/types/schemas';
import * as z from 'zod';
import type { PageServerLoad } from './$types';

type Response<T> = { success: true; result: T } | { success: false; error: string };

export const load: PageServerLoad = async ({
	fetch
}): Promise<Response<GetHistoryResponse[]>> => {
	const res = await fetch(`http://127.0.0.1:3000/history`, {
		credentials: 'include'
	});
	if (!res.ok) return { success: false, error: JSON.stringify(await res.json()) };

	const result = z.array(GetHistoryResponseSchema).safeParse(await res.json());
	if (!result.success) return { success: false, error: 'Failed to parse history response' };

	return { success: true, result: result.data };
};
