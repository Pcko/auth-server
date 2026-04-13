// import type { PageServerLoad } from './$types';
// import { PUBLIC_SERVER_URL } from '$env/static/public';
// import { error } from '@sveltejs/kit';
//
// export const load: PageServerLoad = async ({ fetch, request }) => {
// 	let res = await fetch(`${PUBLIC_SERVER_URL}/auth/me`, {
// 		headers: {
// 			cookie: request.headers.get('cookie') ?? ''
// 		}
// 	});
//
// 	if (!res.ok) {
// 		throw error(res.status, 'Could not load account');
// 	}
//
// 	const user = await res.json();
// 	return { user };
// };
