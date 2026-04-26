import type {PageLoad} from '../../../../.svelte-kit/types/src/routes';
import type {User} from '$lib/types/user';
import {PUBLIC_SERVER_URL} from '$env/static/public';
import { fetchWithRefreshToken, requireAuthOrRedirect } from "$lib/auth-client";

type AuthMeResponse = {
    user: User;
};

export const ssr = false;
export const load: PageLoad = async ({fetch, url}) => {
		const res = await fetchWithRefreshToken(fetch,`${PUBLIC_SERVER_URL}/auth/me`,{
			method: 'GET',
		});

		requireAuthOrRedirect(res,url.pathname + url.search);

    const user: AuthMeResponse = await res.json();
    return {user};
};