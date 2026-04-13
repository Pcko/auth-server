<svelte:head>
    <title>Register</title>
</svelte:head>

<script lang="ts">
    import {PUBLIC_SERVER_URL} from '$env/static/public';
    import {goto} from "$app/navigation";

    let username = '';
    let email = '';
    let password = '';

    async function submit() {
        const response = await fetch(`${PUBLIC_SERVER_URL}/auth/register`, {
            method: 'POST',
            credentials: 'include',
            body: JSON.stringify({username, email, password}),
            headers: {
                'Content-Type': 'application/json'
            }
        });
        console.log(response)
        if (response.status === 201) {
            await goto('/login');
        }
    }
</script>

<div class="min-h-screen flex justify-center items-center p-6">
    <div class="w-full max-w-md rounded-xl border p-6 shadow-sm">
        <h1 class="text-2xl font-semibold">Register</h1>
        <p class="mt-2 text-sm text-muted-foreground">Create your account.</p>

        <form class="mt-6 space-y-4" onsubmit={submit}>
            <div>
                <label class="mb-1 block text-sm font-medium" for="username">Username</label>
                <input
                        id="username"
                        type="text"
                        bind:value={username}
                        class="w-full rounded-lg border px-3 py-2"
                        placeholder="Infinite Shadowz"
                />
            </div>

            <div>
                <label class="mb-1 block text-sm font-medium" for="email">Email</label>
                <input
                        id="email"
                        type="email"
                        bind:value={email}
                        class="w-full rounded-lg border px-3 py-2"
                        placeholder="name@example.com"
                />
            </div>

            <div>
                <label class="mb-1 block text-sm font-medium" for="password">Password</label>
                <input
                        id="password"
                        type="password"
                        bind:value={password}
                        class="w-full rounded-lg border px-3 py-2"
                        placeholder="••••••••"
                >
            </div>

            <button type="submit" class="w-full rounded-lg border px-4 py-2 font-medium hover:bg-secondary">
                Register
            </button>
        </form>
        <p class="mt-4 text-sm">
            Already have an Account?
            <a href="/login" class="underline">Log in</a>
        </p>
    </div>
</div>