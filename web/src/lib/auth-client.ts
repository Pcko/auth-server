import {PUBLIC_SERVER_URL} from "$env/static/public";
import { redirect, error } from '@sveltejs/kit';

export async function refreshAccessToken(fetcher: typeof fetch) {
    const res = await fetcher(`${PUBLIC_SERVER_URL}/auth/refresh`, {
        method: 'POST',
        credentials: 'include',
    });

    return res.ok;
}

export async function fetchWithRefreshToken(fetcher: typeof fetch,  input: string,
                                            init: RequestInit = {}) {
    const send = () =>
        fetcher(input, {
            ...init,
            credentials: 'include'
        });

    let res = await send();

    if (res.status === 401) {
        const refreshed = await refreshAccessToken(fetcher);

        if (refreshed) {
            res = await send();
        }
    }

    return res;
}

export function requireAuthOrRedirect(res: Response, returnTo: string) {
    if (res.status === 401) {
        throw redirect(303, `/login?return_to=${encodeURIComponent(returnTo)}`);
    }

    if (!res.ok) {
        throw error(res.status, 'Request failed');
    }
}