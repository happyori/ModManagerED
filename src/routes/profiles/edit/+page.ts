import type { PageLoad } from './$types';

export const load = (async ({ url }) => {
	const id = url.searchParams.get('id');

	if (id === null) {
		throw new Error('Failed to extract id from query params!');
	}

	return { id };
}) satisfies PageLoad;
