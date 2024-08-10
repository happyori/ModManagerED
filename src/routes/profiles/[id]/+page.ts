import type { PageLoad } from './$types';

export const load = (async ({ params }) => {
	let { id } = params;
	return { id };
}) satisfies PageLoad;
