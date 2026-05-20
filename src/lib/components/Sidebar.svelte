<script lang="ts">
    import { navStore } from "$lib/stores/nav.svelte";
    import { modStore } from "$lib/stores/mods.svelte";
    import { updaterStore } from "$lib/stores/updater.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import {
        Puzzle,
        Download,
        Settings,
        ScrollText,
        Play,
    } from "@lucide/svelte";
    import logo from "$lib/assets/logo.png";
    import { invoke } from "@tauri-apps/api/core";

    const items = [
        { id: "mods" as const, label: "Mods", icon: Puzzle },
        { id: "browse" as const, label: "Browse", icon: Download },
        { id: "logs" as const, label: "Logs", icon: ScrollText },
        { id: "settings" as const, label: "Settings", icon: Settings },
    ];

    function launch() {
        invoke("launch_game");
    }
</script>

<div class="w-[150px] border-r border-border flex flex-col bg-card shrink-0">
    <div class="px-4 py-4 border-b border-border">
        <div class="flex items-center gap-2.5">
            <img class="size-7 rounded-md" src={logo} />
            <!-- <div
                class="size-7 rounded-md bg-primary flex items-center justify-center"
            >
                <span class="text-primary-foreground text-xs font-bold">C</span>
            </div> -->
            <div>
                <p class="text-sm font-medium leading-none">Cantus</p>
                <p class="text-[10px] text-muted-foreground mt-0.5">
                    v{updaterStore.currentVersion || "?"}
                </p>
            </div>
        </div>
    </div>

    <nav class="flex-1 p-2">
        {#each items as item}
            {@const active = navStore.active === item.id}
            <Button
                variant="ghost"
                class="w-full justify-start gap-2.5 px-3 py-2 h-auto text-sm mb-0.5 {active
                    ? 'bg-accent text-accent-foreground font-medium hover:bg-accent'
                    : 'text-muted-foreground hover:text-foreground hover:bg-accent/50'}"
                onclick={() => (navStore.active = item.id)}
            >
                <item.icon class="size-4" />
                <span>{item.label}</span>
                {#if item.id === "mods"}
                    <span
                        class="ml-auto text-[11px] tabular-nums {active
                            ? 'text-accent-foreground/70'
                            : 'text-muted-foreground/60'}"
                    >
                        {modStore.enabledCount}/{modStore.totalCount}
                    </span>
                {/if}
            </Button>
        {/each}
    </nav>

    <div class="p-2 border-t border-border">
        <Button class="w-full justify-start gap-2.5" size="sm" onclick={launch}>
            <Play class="size-3.5" />
            Launch MSM
        </Button>
    </div>
</div>
