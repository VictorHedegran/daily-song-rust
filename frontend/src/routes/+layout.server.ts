import { UserResponseSchema, type UserResponse } from '$lib/types/schemas';
import type { LayoutServerLoad } from './$types';
import { BACKEND_URL } from '$env/static/private';

export const load: LayoutServerLoad = ({ fetch }) => {
	const user: Promise<UserResponse | null> = fetch(`${BACKEND_URL}/me`, {
		credentials: 'include'
	})
		.then(async (res) => {
			if (!res.ok) return null;
			const result = UserResponseSchema.safeParse(await res.json());
			return result.success ? result.data : null;
		})
		.catch(() => null);

	return { user };
};
