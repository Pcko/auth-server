import type {PageLoad} from "../../../../.svelte-kit/types/src/routes/$types";
import {PUBLIC_SERVER_URL} from "$env/static/public";
import { fetchWithRefreshToken, requireAuthOrRedirect } from "$lib/auth-client";
import type {Session} from "$lib/types/session";

export const ssr = false;

export const load: PageLoad = async ({fetch,url}) => {
 		const res = await fetchWithRefreshToken(fetch,`${PUBLIC_SERVER_URL}/sessions`,{
			 method: "GET",
		});

		requireAuthOrRedirect(res, url.pathname + url.search);

    const data: Session[] = await res.json();
    data.map((session: Session) => {
        session.created_at = new Date(session.created_at);
        session.expires_at = new Date(session.expires_at);
        session.last_seen_at = new Date(session.last_seen_at)
        return session;
    });
    return { sessions: data};
}