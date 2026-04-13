<svelte:head>
    <title>Account</title>
</svelte:head>

<script lang="ts">
    import { Button } from '$lib/components/ui/button/index.js';
    import type { PageData } from './$types';
    import { PUBLIC_SERVER_URL } from '$env/static/public';

    let { data }: { data: PageData } = $props();

    let username = $state<string>(data.user?.username ?? '');
    let email = $state<string>(data.user?.email ?? '');
    let mfa = $state<boolean>(data.user?.mfa ?? '');

    let password = $state<string>('');
    let confirmPassword = $state<string>('');
    let error = $state<string>('');
    let success = $state<string>('');
    let pending = $state<boolean>(false);

    async function submit() {
        error = '';
        success = '';

        if (password !== confirmPassword) {
            error = 'Passwords do not match.';
            return;
        }

        pending = true;

        try {
            const response = await fetch(`${PUBLIC_SERVER_URL}/users/me`, {
                method: 'PATCH',
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    username: username.trim() || null,
                    email: email.trim() || null,
                    mfa,
                    password: password.trim() || null
                })
            });

            if (!response.ok) {
                const body = await response.json().catch(() => null);
                error = body?.message ?? 'Failed to update account.';
                console.log(response.statusText)
                return;
            }

            success = 'Account updated successfully.';
            password = '';
            confirmPassword = '';
        } catch {
            error = 'Request failed.';
        } finally {
            pending = false;
        }
    }
</script>

<div class="flex w-full justify-center p-4 md:p-6">
    <form class="w-full max-w-4xl space-y-6" on:submit|preventDefault={submit}>
        <div class="space-y-1">
            <h1 class="text-2xl font-semibold tracking-tight text-foreground">Account Settings</h1>
            <p class="text-sm text-muted-foreground">Manage your profile information and account security.</p>
        </div>

        <section class="rounded-none border border-border bg-card p-6 shadow-sm">
            <div class="mb-6 space-y-1">
                <h2 class="text-lg font-semibold text-card-foreground">Profile</h2>
                <p class="text-sm text-muted-foreground">Update your basic account information.</p>
            </div>

            <div class="grid gap-4 md:grid-cols-2">
                <div class="space-y-2">
                    <label for="username" class="text-sm font-medium text-foreground">Username</label>
                    <input
                            id="username"
                            type="text"
                            bind:value={username}
                            class="flex h-10 w-full rounded-none border border-input bg-background px-3 py-2 text-sm text-foreground shadow-xs outline-none transition-colors placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                    />
                </div>

                <div class="space-y-2">
                    <label for="email" class="text-sm font-medium text-foreground">Email</label>
                    <input
                            id="email"
                            type="email"
                            bind:value={email}
                            class="flex h-10 w-full rounded-none border border-input bg-background px-3 py-2 text-sm text-foreground shadow-xs outline-none transition-colors placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                    />
                </div>
            </div>
        </section>

        <section class="rounded-none border border-border bg-card p-6 shadow-sm">
            <div class="mb-6 space-y-1">
                <h2 class="text-lg font-semibold text-card-foreground">Security</h2>
                <p class="text-sm text-muted-foreground">Change your password and manage additional protection.</p>
            </div>

            <div class="space-y-6">
                <div class="grid gap-4 md:grid-cols-2">
                    <div class="space-y-2">
                        <label for="password" class="text-sm font-medium text-foreground">New Password</label>
                        <input
                                id="password"
                                type="password"
                                bind:value={password}
                                placeholder="Enter a new password"
                                class="flex h-10 w-full rounded-none border border-input bg-background px-3 py-2 text-sm text-foreground shadow-xs outline-none transition-colors placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                        />
                    </div>

                    <div class="space-y-2">
                        <label for="confirmPassword" class="text-sm font-medium text-foreground">Confirm Password</label>
                        <input
                                id="confirmPassword"
                                type="password"
                                bind:value={confirmPassword}
                                placeholder="Repeat your new password"
                                class="flex h-10 w-full rounded-none border border-input bg-background px-3 py-2 text-sm text-foreground shadow-xs outline-none transition-colors placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                        />
                    </div>
                </div>

                <div class="flex items-start justify-between gap-4 rounded-none border border-border bg-background/60 p-4">
                    <div class="space-y-1">
                        <label for="mfa" class="text-sm font-medium text-foreground">Enable MFA</label>
                        <p class="text-sm text-muted-foreground">Add an extra layer of security to your account.</p>
                    </div>

                    <label for="mfa" class="relative inline-flex cursor-pointer items-center">
                        <input id="mfa" type="checkbox" bind:checked={mfa} class="peer sr-only" />
                        <div class="h-6 w-11 rounded-none bg-muted transition-colors duration-200 peer-checked:bg-primary peer-focus-visible:ring-2 peer-focus-visible:ring-ring peer-focus-visible:ring-offset-2"></div>
                        <div class="absolute left-0.5 top-0.5 h-5 w-5 rounded-none bg-background shadow-sm transition-transform duration-200 peer-checked:translate-x-5"></div>
                    </label>
                </div>
            </div>
        </section>

        <div class="flex items-start justify-between gap-4">
            <div class="min-h-5">
                {#if error}
                    <p class="text-sm text-destructive">{error}</p>
                {:else if success}
                    <p class="text-sm text-green-600">{success}</p>
                {/if}
            </div>

            <Button type="submit" class="rounded-none px-5 shrink-0" disabled={pending}>
                {pending ? 'Saving...' : 'Save Changes'}
            </Button>
        </div>
    </form>
</div>