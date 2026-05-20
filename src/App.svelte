<script lang="ts">
    import { onMount } from "svelte";
    import Sidebar from "$lib/components/Sidebar.svelte";
    import ModsTab from "$lib/components/tabs/ModsTab.svelte";
    import BrowseTab from "$lib/components/tabs/BrowseTab.svelte";
    import LogsTab from "$lib/components/tabs/LogsTab.svelte";
    import SettingsTab from "$lib/components/tabs/SettingsTab.svelte";
    import Onboarding from "$lib/components/onboarding/Onboarding.svelte";
    import UpdateDialog from "$lib/components/UpdateDialog.svelte";
    import { Toaster } from "$lib/components/ui/sonner";
    import { navStore } from "$lib/stores/nav.svelte";
    import { setupStore } from "$lib/stores/setup.svelte";
    import { updaterStore } from "$lib/stores/updater.svelte";

    onMount(() => {
        setupStore.loadSettings();
        updaterStore.init();
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

<Toaster richColors />
<UpdateDialog />
