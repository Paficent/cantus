<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import { Badge } from "$lib/components/ui/badge";
    import Button from "$lib/components/ui/button/button.svelte";
    import {
        Download,
        Eye,
        Heart,
        Loader2,
        Calendar,
        RefreshCw,
    } from "@lucide/svelte";
    import type { BrowseMod } from "$lib/types/browse";
    import { invoke } from "@tauri-apps/api/core";

    let {
        mod,
        installing,
        oninstall,
    }: {
        mod: BrowseMod;
        installing: boolean;
        oninstall: (id: number) => void;
    } = $props();

    function fmt(n: number) {
        if (n >= 1000) return (n / 1000).toFixed(1).replace(/\.0$/, "") + "k";
        return n.toString();
    }

    function since(ts: number) {
        if (!ts) return "Unknown";
        const s = Math.floor(Date.now() / 1000 - ts);
        if (s < 60) return "just now";
        if (s < 3600) return Math.floor(s / 60) + "m ago";
        if (s < 86400) return Math.floor(s / 3600) + "h ago";
        if (s < 86400 * 30) return Math.floor(s / 86400) + "d ago";
        if (s < 86400 * 365) return Math.floor(s / 86400 / 30) + "mo ago";
        return Math.floor(s / 86400 / 365) + "y ago";
    }

    const url = $derived(`https://gamebanana.com/mods/${mod.id}`);
</script>

<Card.Root class="relative pt-0 transition-colors hover:border-foreground/20">
    <div class="relative w-full aspect-video overflow-hidden">
        <button
            type="button"
            class="cursor-pointer border-0 bg-transparent p-0 m-0 block"
            onclick={() => invoke("open_external_url", { url })}
        >
            <img
                src={mod.screenshot}
                alt={mod.name}
                loading="lazy"
                class="object-cover"
            />
        </button>
        <div class="absolute top-2 left-2">
            <Badge
                variant="secondary"
                class="bg-background/80 backdrop-blur-sm text-[11px]"
            >
                {mod.category}
            </Badge>
        </div>
    </div>

    <Card.Header>
        <Card.Title
            class="truncate"
            onclick={() => invoke("open_external_url", { url })}
            >{mod.name}</Card.Title
        >
        <Card.Description>{mod.author}</Card.Description>
    </Card.Header>

    <Card.Content class="pt-0">
        <div class="flex items-center gap-3 text-[11px] text-muted-foreground">
            <span class="flex items-center gap-1">
                <Eye class="size-3" />
                {fmt(mod.views)}
            </span>
            <span class="flex items-center gap-1">
                <Heart class="size-3" />
                {fmt(mod.likes)}
            </span>
            {#if mod.downloads}
                <span class="flex items-center gap-1">
                    <Download class="size-3" />
                    {fmt(mod.downloads)}
                </span>
            {/if}
        </div>
        <div
            class="flex items-center gap-3 mt-1.5 text-[11px] text-muted-foreground"
        >
            <span class="flex items-center gap-1" title="Created">
                <Calendar class="size-3" />
                {since(mod.date_added)}
            </span>
            {#if mod.date_updated && mod.date_updated !== mod.date_added}
                <span class="flex items-center gap-1" title="Last updated">
                    <RefreshCw class="size-3" />
                    {since(mod.date_updated)}
                </span>
            {/if}
        </div>
    </Card.Content>

    <Card.Footer>
        <Button
            variant="outline"
            size="sm"
            class="w-full"
            disabled={installing}
            onclick={() => oninstall(mod.id)}
        >
            {#if installing}
                <Loader2 class="size-3.5 animate-spin" />
                Installing...
            {:else}
                <Download class="size-3.5" />
                Install
            {/if}
        </Button>
    </Card.Footer>
</Card.Root>
