<script lang="ts">
    import { onMount } from "svelte";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import CantusCard from "$lib/components/settings/CantusCard.svelte";
    import JeodeCard from "$lib/components/settings/JeodeCard.svelte";
    import { Loader2 } from "@lucide/svelte";

    onMount(() => {
        settingsStore.load();
        settingsStore.startWatching();
    });
</script>

<div class="flex flex-col h-full">
    <div class="flex items-start justify-between mb-4">
        <div>
            <h2 class="text-base font-medium">Settings</h2>
        </div>
    </div>

    {#if settingsStore.loading}
        <div
            class="flex flex-col items-center justify-center flex-1 text-muted-foreground"
        >
            <Loader2 class="size-10 mb-3 opacity-30 animate-spin" />
            <p class="text-sm">Loading settings...</p>
        </div>
    {:else}
        <div class="flex-1 overflow-y-auto pr-1 space-y-4">
            <CantusCard />
            <JeodeCard />
        </div>
    {/if}
</div>
