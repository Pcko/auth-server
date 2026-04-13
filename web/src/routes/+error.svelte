<script lang="ts">
    import { page } from '$app/state';
</script>

<svelte:head>
    <title>Error</title>
</svelte:head>

<div class="flex min-h-screen items-center justify-center p-6">
    <div class="max-w-2xl mb-20 ">
        <h1 class="text-9xl font-semibold">
            {page.status}
        </h1>

        <p class="mt-2 text-xl text-destructive">
            {page.error?.message ?? 'Something went wrong.'}
        </p>

        <a href="/" class="mt-4 inline-block underline">
            Back to home
        </a>
    </div>
</div>