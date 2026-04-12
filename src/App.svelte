<script lang="ts">
    import { Sidebar } from "$lib/components/layout";
    import {
        ModsTab,
        BrowseTab,
        LogsTab,
        SettingsTab,
    } from "$lib/components/tabs";
    import { Onboarding } from "$lib/components/onboarding";
    import { navStore } from "$lib/stores/nav.svelte";
    import { setupStore } from "$lib/stores/setup.svelte";

    $effect(() => {
        setupStore.loadSettings();
    });
</script>

{#if !setupStore.loaded}
    <div class="flex h-screen items-center justify-center select-none">
        <p class="text-sm text-muted-foreground">Loading...</p>
    </div>
{:else if !setupStore.complete}
    <Onboarding />
{:else}
    <div class="flex h-screen overflow-hidden select-none">
        <Sidebar />

        <main class="flex-1 p-6 overflow-hidden">
            {#if navStore.active === "mods"}
                <ModsTab />
            {:else if navStore.active === "browse"}
                <BrowseTab />
            {:else if navStore.active === "settings"}
                <SettingsTab />
            {:else if navStore.active === "logs"}
                <LogsTab />
            {/if}
        </main>
    </div>
{/if}
