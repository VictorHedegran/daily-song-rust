import type { Actions } from '@sveltejs/kit';

export const actions: Actions = {
	logout: async ({ fetch }) => {
		const res = await fetch('http://127.0.0.1:3000/logout', {
			method: 'POST',
			credentials: 'include'
		});
		if (!res.ok) return { success: false, error: res.statusText };

		return { success: true, result: undefined };
	},
};
