import type { Actions } from '@sveltejs/kit';
import { BACKEND_URL } from '$env/static/private';

export const actions: Actions = {
	logout: async ({ fetch }) => {
		const res = await fetch(`${BACKEND_URL}/logout`, {
			method: 'POST',
			credentials: 'include'
		});
		if (!res.ok) return { success: false, error: res.statusText };

		return { success: true, result: undefined };
	}
};
