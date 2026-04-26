<svelte:head>
    <title>Register</title>
</svelte:head>

<script lang="ts">
    import {PUBLIC_SERVER_URL} from '$env/static/public';
    import {goto} from "$app/navigation";
    import ThemeButton from "$lib/components/ui/theme-button/theme-button.svelte";

    let username = $state('');
    let email = $state('');
    let password = $state('');

    let loading = $state(false);
    let error = $state<string | null>(null);


    async function submit(event : SubmitEvent) {
        event.preventDefault();

        try {
            loading = true;
            error = null;

            const response = await fetch(`${PUBLIC_SERVER_URL}/auth/register`, {
                method: 'POST',
                credentials: 'include',
                body: JSON.stringify({username, email, password}),
                headers: {
                    'Content-Type': 'application/json'
                }
            });

            if (response.status === 201) {
                await goto('/login');
                return;
            }

            error = `Error: ${response.statusText}`;
        } catch {
            error = 'Network Error. Please try again Later';
        } finally {
            loading = false;
        }
    }
</script>

<div class="min-h-screen flex justify-center items-center p-6">
    <div class="w-full max-w-md rounded-none border p-6 shadow-sm bg-card">
        <h1 class="text-2xl font-semibold">Register</h1>
        <p class="mt-2 text-sm text-muted-foreground">Create your account.</p>

        <form class="mt-6 space-y-4" onsubmit={submit}>
            <div>
                <label class="mb-1 block text-sm font-medium" for="username">Username</label>
                <input
                        id="username"
                        type="text"
                        bind:value={username}
                        class="w-full rounded-none border px-3 py-2 bg-background"
                        placeholder="Infinite Shadowz"
                />
            </div>

            <div>
                <label class="mb-1 block text-sm font-medium" for="email">Email</label>
                <input
                        id="email"
                        type="email"
                        bind:value={email}
                        class="w-full rounded-none border px-3 py-2 bg-background"
                        placeholder="name@example.com"
                />
            </div>

            <div>
                <label class="mb-1 block text-sm font-medium" for="password">Password</label>
                <input
                        id="password"
                        type="password"
                        bind:value={password}
                        class="w-full rounded-none border px-3 py-2 bg-background"
                        placeholder="••••••••"
                >
            </div>

            {#if error}
                <p class="text-sm text-destructive transition-transform" role="alert">
                    {error}
                </p>
            {/if}

            <button type="submit" disabled={loading}
                    class="w-full rounded-none border px-4 py-2 font-medium hover:bg-secondary bg-background"
            >
                Register
            </button>
        </form>
        <p class="mt-4 text-sm">
            Already have an Account?
            <a href="/login" class="underline">Log in</a>
        </p>
    </div>
</div>

<!--Theme Switch Button -->
<ThemeButton/>