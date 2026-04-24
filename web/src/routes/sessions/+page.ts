import type {PageLoad} from "../../../.svelte-kit/types/src/routes/$types";
import {PUBLIC_SERVER_URL} from "$env/static/public";
import {refreshAccessToken} from "$lib/auth-client";
import {error} from "@sveltejs/kit";
import type {Session} from "$lib/types/session";

export const ssr = false;

export const load: PageLoad = async ({fetch}) => {
    const sendRequest = async () => await fetch(`${PUBLIC_SERVER_URL}/sessions`, {
        method: "GET",
        credentials: "include",
    });

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

    const sessions: Session[] = await res.json();
    sessions.map((session: Session) => {
        session.created_at = new Date(session.created_at);
        session.expires_at = new Date(session.expires_at);
        session.last_seen_at = new Date(session.last_seen_at)
        return session;
    });
    return {sessions};
}