<script lang="ts">
    import { navStore, type Tab } from "$lib/stores/nav.svelte";
    import { modStore } from "$lib/stores/mods.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import { Puzzle, Download, Settings, ScrollText, Play } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Component } from "svelte";

    interface NavItem {
        id: Tab;
        label: string;
        icon: Component;
    }

    const items: NavItem[] = [
        { id: "mods", label: "Mods", icon: Puzzle },
        { id: "browse", label: "Browse", icon: Download },
        { id: "logs", label: "Logs", icon: ScrollText },
        { id: "settings", label: "Settings", icon: Settings },
    ];
</script>

<div class="w-[150px] border-r border-border flex flex-col bg-card shrink-0">
    <div class="px-4 py-4 border-b border-border">
        <div class="flex items-center gap-2.5">
            <div
                class="size-7 rounded-md bg-primary flex items-center justify-center"
            >
                <span class="text-primary-foreground text-xs font-bold">C</span>
            </div>
            <div>
                <p class="text-sm font-medium leading-none">Cantus</p>
                <p class="text-[10px] text-muted-foreground mt-0.5">meow</p>
            </div>
        </div>
    </div>

    <nav class="flex-1 p-2">
        {#each items as item}
            <Button
                variant="ghost"
                class="w-full justify-start gap-2.5 px-3 py-2 h-auto text-sm mb-0.5
          {navStore.active === item.id
                    ? 'bg-accent text-accent-foreground font-medium hover:bg-accent'
                    : 'text-muted-foreground hover:text-foreground hover:bg-accent/50'}"
                onclick={() => {
                    navStore.active = item.id;
                }}
            >
                <item.icon class="size-4" />
                <span>{item.label}</span>
                {#if item.id === "mods"}
                    <span
                        class="ml-auto text-[11px] tabular-nums {navStore.active ===
                        item.id
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
        <Button
            class="w-full justify-start gap-2.5"
            size="sm"
            onclick={() => invoke("launch_game")}
        >
            <Play class="size-3.5" />
            Launch MSM
        </Button>
    </div>
</div>
