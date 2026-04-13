import {PUBLIC_SERVER_URL} from "$env/static/public";

export async function refreshAccessToken(fetcher: typeof fetch) {
    const res = await fetcher(`${PUBLIC_SERVER_URL}/auth/refresh`, {
        method: 'POST',
        credentials: 'include',
    });

    return res.ok;
}