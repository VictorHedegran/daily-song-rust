import { UserResponseSchema, type UserResponse } from '$lib/types/schemas';
import type { LayoutServerLoad } from './$types';

type Response<T> = { success: true; result: T } | { success: false; error: string };

export const load: LayoutServerLoad = async ({ fetch }): Promise<Response<UserResponse>> => {
	const res = await fetch('http://127.0.0.1:3000/me', {
		credentials: 'include'
	});
	if (!res.ok) return { success: false, error: res.statusText };

	const result = UserResponseSchema.safeParse(await res.json());
	if (!result.success) return { success: false, error: 'Failed to parse user response' };

	return { success: true, result: result.data };
};
