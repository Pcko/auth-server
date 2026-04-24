<script lang="ts">
    import {toggleMode,mode} from 'mode-watcher'
    import SunIcon from '../../../assets/SunIcon.svelte'
    import MoonIcon from '../../../assets/MoonIcon.svelte'
</script>


<button onclick={toggleMode}
        class="fixed bottom-8 right-9 z-50 inline-flex h-15 w-15 items-center justify-center border bg-card">
    {#if mode.current==="light"}
        <SunIcon/>
    {:else}
        <MoonIcon/>
    {/if}
</button>