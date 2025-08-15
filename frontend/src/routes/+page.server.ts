import type { ServerLoad } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

export const load: ServerLoad = async ({ fetch }) => {
	let base = env.API_URL ?? '';

	// normalize base: ensure it has a scheme if provided (avoid 'unknown scheme')
	if (base) {
		base = base.trim();
		// remove trailing slash
		base = base.replace(/\/+$/g, '');
		if (!/^https?:\/\//i.test(base)) {
			base = `http://${base}`; // assume http if no scheme provided
		}
	}

	const url = base ? `${base}/wage/data` : '/wage/data';

	try {
		const res = await fetch(url);
		if (!res.ok) return { wageData: null };
		const wageData = await res.json();
		return { wageData };
	} catch (err) {
		// network or invalid URL
		console.error('fetch error in +page.server.load', err);
		return { wageData: null };
	}
};