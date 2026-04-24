<script lang="ts">
    import type {PageData} from './$types';
    import type {Session} from '$lib/types/session';
    import {PUBLIC_SERVER_URL} from "$env/static/public";

    let {data}: { data: PageData } = $props();

    let sessions = $state<Session[]>(data.sessions ?? []);
    let searchQuery = $state('');
    let error = $state<string | null>(null);

    let sortedSessions = $derived(
        [...sessions].sort(
            (a, b) => b.created_at.getTime() - a.created_at.getTime()
        )
    );

    function getStatus(session: Session): 'active' | 'expired' | 'revoked' {
        if (session.revoked_at) return 'revoked';
        if (session.expires_at.getTime() < Date.now()) return 'expired';
        return 'active';
    }

    function shortId(id: string) {
        return `${id.slice(0, 6)}…${id.slice(-4)}`;
    }

    function formatDate(date: Date | null) {
        if (!date) return '—';

        return new Intl.DateTimeFormat('de-DE', {
            dateStyle: 'medium',
            timeStyle: 'short'
        }).format(date);
    }

    function parseUserAgent(ua: string | null) {
        if (!ua) return 'Unknown device';

        const browser =
            ua.match(/Firefox\/(\d+)/)?.[0] ??
            ua.match(/Chrome\/(\d+)/)?.[0] ??
            ua.match(/Safari\/(\d+)/)?.[0] ??
            'Unknown browser';

        const os =
            ua.includes('Linux') ? 'Linux' :
                ua.includes('Windows') ? 'Windows' :
                    ua.includes('Mac OS X') ? 'macOS' :
                        'Unknown OS';

        return `${browser} · ${os}`;
    }

    async function revokeSession(session: Session) {
        try {
            const response = await fetch(`${PUBLIC_SERVER_URL}/sessions/${session.id}`, {
                method: 'DELETE',
                credentials: 'include',
            });

            if (response.status === 204) {
                sessions = sessions.filter(s => s.id !== session.id);
                return;
            }

            error = `Error: ${response.statusText}`;
        } catch (err) {
            console.error(err);
            error = "Network Error. Please try again later!";
        }
    }
</script>

<div class="flex w-full justify-center p-4 md:p-6">
    <div class="w-full max-w-4xl space-y-6">
        <div class="space-y-1">
            <h1 class="text-2xl font-semibold tracking-tight text-foreground">
                Session Management
            </h1>
            <p class="text-sm text-muted-foreground">
                Manage your active sessions.
            </p>
        </div>

        <div class="overflow-hidden border bg-card">
            <!--TODO implement Search/>-->
            <div class="max-w-md mx-auto my-3 bg-background">
                <label for="search" class="block mb-2.5 text-sm font-medium text-heading sr-only ">Search...</label>
                <div class="relative">
                    <div class="absolute inset-y-0 inset-s-0 flex items-center ps-3 pointer-events-none">
                        <svg class="w-4 h-4 text-body" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24"
                             height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-width="2"
                                  d="m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z"/>
                        </svg>
                    </div>
                    <input value={searchQuery} type="search" id="search"
                           class="block w-full p-3 ps-9 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body"
                           placeholder="Search" required/>
                </div>
            </div>


            {#if sortedSessions.length === 0}
                <div class="p-6 text-sm text-muted-foreground">
                    No sessions found.
                </div>
            {:else}
                <div class="divide-y">
                    {#each sortedSessions as session (session.id)}
                        {@const status = getStatus(session)}

                        <div class="flex flex-col gap-4 p-4 md:flex-row md:items-center md:justify-between">
                            <div class="min-w-0 space-y-2">
                                <div class="flex flex-wrap items-center gap-2">
                                    <p class="font-medium text-foreground">
                                        {parseUserAgent(session.user_agent)}
                                    </p>
                                </div>

                                <div class="grid gap-1 text-sm text-muted-foreground pl-2">
                                    <p><span class="font-medium text-foreground">Session:</span> {shortId(session.id)}
                                    </p>
                                    <p><span class="font-medium text-foreground">User:</span> {shortId(session.uid)}</p>
                                    <p><span
                                            class="font-medium text-foreground">Created:</span> {formatDate(session.created_at)}
                                    </p>
                                    <p><span
                                            class="font-medium text-foreground">Last seen:</span> {formatDate(session.last_seen_at)}
                                    </p>
                                    <p><span
                                            class="font-medium text-foreground">Expires:</span> {formatDate(session.expires_at)}
                                    </p>
                                    <p><span class="font-medium text-foreground">IP:</span> {session.ip_address ?? '—'}
                                    </p>
                                </div>
                            </div>

                            <div class="flex shrink-0 items-center gap-2">
                                <button
                                        type="button"
                                        class="rounded-none border px-3 py-2 text-sm bg-background hover:bg-accent"
                                >
                                    View
                                </button>

                                {#if status === 'active'}
                                    <button
                                            onclick={() => revokeSession(session)}
                                            type="button"
                                            class="rounded-none border px-3 py-2 text-sm bg-background hover:bg-accent"
                                    >
                                        Revoke
                                    </button>
                                {/if}
                            </div>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    </div>
</div>