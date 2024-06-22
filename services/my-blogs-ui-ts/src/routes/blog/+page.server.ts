import type { BlogData } from '$lib/types';

export function load(): BlogData {
	const data: BlogData = { summaries: [{ title: 'Hello', body: 'World', slug: 'hello-world' }] };
	return data;
}
