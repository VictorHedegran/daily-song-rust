import type { HandleFetch } from '@sveltejs/kit';
import { BACKEND_URL } from '$env/static/private';

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	if (request.url.startsWith(BACKEND_URL)) {
		const cookie = event.request.headers.get('cookie');
		if (cookie) {
			request.headers.set('cookie', cookie);
		}
	}
	return fetch(request);
};