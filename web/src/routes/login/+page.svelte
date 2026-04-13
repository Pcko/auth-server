<!-- src/routes/login/+page.svelte -->
<svelte:head>
    <title>Login</title>
</svelte:head>

<script lang="ts">
    import {PUBLIC_SERVER_URL} from "$env/static/public";

    let email = $state('');
    let password = $state('');
    let loading = $state(false)
    let error = $state<string | null>(null);

    async function submit() {
        error = null;
        loading = true;

        try {
            const response = await fetch(`${PUBLIC_SERVER_URL}/auth/login`, {
                method: 'POST',
                credentials: 'include',
                body: JSON.stringify({email, password}),
                headers: {
                    'Content-Type': 'application/json'
                }
            })
            console.log(response)

            if (response.status === 200) {
                const data = await response.json();
                console.log(data)
                return;
            }

            error = `Error: ${response.statusText}`;
        } catch (err) {
            error = 'Network Error. Please try again Later';
        } finally {
            loading = false;
        }
    }
</script>

<div class="min-h-screen flex items-center justify-center p-">
    <div class="w-full max-w-md rounded-none border p-6 shadow-sm bg-card">
        <h1 class="text-2xl font-semibold">Login</h1>
        <p class="mt-2 text-sm text-muted-foreground">Log into your Account.</p>

        <form class="mt-6 space-y-4" onsubmit={submit}>
            <div>
                <label class="mb-1 block text-sm font-medium" for="email">E-Mail</label>
                <input
                        id="email"
                        type="email"
                        bind:value={email}
                        class="w-full rounded-none border px-3 py-2 bg-background"
                        placeholder="name@example.com"
                />
            </div>

            <div>
                <label class="mb-1 block text-sm font-medium" for="password">Passwort</label>
                <input
                        id="password"
                        type="password"
                        bind:value={password}
                        class="w-full rounded-none border px-3 py-2 bg-background"
                        placeholder="••••••••"
                />
            </div>
            {#if error}
                <p class="text-sm text-destructive" role="alert">
                    {error}
                </p>
            {/if}

            <button
                    type="submit"
                    disabled={loading}
                    class="w-full rounded-none border px-4 py-2 font-medium hover:bg-secondary disabled:opacity-50 bg-background"
            >
                Log-In
            </button>
        </form>

        <p class="mt-4 text-sm">
            Don't have an account?
            <a href="/register" class="underline">Register</a>
        </p>
    </div>
</div>