import type { UserResponse } from '$lib/types/UserResponse';

export const load = async ({ fetch }: { fetch: typeof window.fetch }) => {
  const res = await fetch('http://127.0.0.1:3000/me', {
    credentials: 'include',
  });
  return { me: (await res.json()) as UserResponse };
};