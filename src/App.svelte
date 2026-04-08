<script lang="ts">
    import { Sidebar } from "$lib/components/layout";
    import {
        ModsTab,
        BrowseTab,
        LogsTab,
        PlaceholderTab,
    } from "$lib/components/tabs";
    import { Onboarding } from "$lib/components/onboarding";
    import { navStore } from "$lib/stores/nav.svelte";
    import { setupStore } from "$lib/stores/setup.svelte";
</script>

{#if !setupStore.complete}
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
                <PlaceholderTab
                    title="Settings"
                    description="Jeode configuration"
                />
            {:else if navStore.active === "logs"}
                <LogsTab />
            {/if}
        </main>
    </div>
{/if}
