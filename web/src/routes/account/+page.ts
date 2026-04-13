import type {PageLoad} from './$types';
import type {User} from '$lib/types/user';
import {PUBLIC_SERVER_URL} from '$env/static/public';
import {error} from "@sveltejs/kit";
import {refreshAccessToken} from "$lib/auth-client";

type AuthMeResponse = {
    user: User;
};

export const ssr = false;
export const load: PageLoad = async ({fetch}) => {
    let sendRequest = () => fetch(`${PUBLIC_SERVER_URL}/auth/me`, {method: 'GET', credentials: 'include',});
    let res = await sendRequest();

    if (res.status === 401) {
        let refreshed = await refreshAccessToken(fetch);

        if (refreshed) {
            res = await sendRequest();
        }
    }

    if (!res.ok) {
        throw error(res.status, 'Could not load account');
    }

    const user: AuthMeResponse = await res.json();
    return {user};
};