<script lang="ts">
    import { onMount } from "svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import Input from "$lib/components/ui/input/input.svelte";
    import * as Select from "$lib/components/ui/select";
    import { LogEntry } from "$lib/components/logs";
    import { logStore } from "$lib/stores/logs.svelte";
    import {
        Search,
        Trash2,
        ScrollText,
        Loader2,
        RefreshCw,
        Clipboard,
        CircleAlert,
    } from "lucide-svelte";
    import type { LevelFilter } from "$lib/types/logs";

    let copied = $state(false);

    const levelLabels: Record<LevelFilter, string> = {
        all: "All levels",
        info: "Info",
        debug: "Debug",
        warn: "Warn",
        error: "Error",
    };
    let scrollContainer = $state<HTMLElement | null>(null);

    onMount(() => {
        logStore.load();
        logStore.startWatching();
    });

    $effect(() => {
        logStore.filtered.length;
        if (scrollContainer) {
            requestAnimationFrame(() => {
                scrollContainer!.scrollTop = scrollContainer!.scrollHeight;
            });
        }
    });

    function handleCopy() {
        logStore.copyToClipboard();
        copied = true;
        setTimeout(() => (copied = false), 1500);
    }
</script>

<div class="flex flex-col h-full">
    <div class="flex items-start justify-between mb-4">
        <div>
            <h2 class="text-base font-medium">Logs</h2>
            <p class="text-xs text-muted-foreground mt-0.5">
                {logStore.filtered.length} of {logStore.counts.total} entries
                <span class="ml-2 text-muted-foreground/50">
                    {logStore.counts.info} info · {logStore.counts.debug} debug ·
                    {logStore.counts.warn} warn · {logStore.counts.error} error
                </span>
            </p>
        </div>
        <div class="flex items-center gap-1.5">
            <Button
                variant="outline"
                size="sm"
                onclick={() => logStore.load()}
                disabled={logStore.loading}
            >
                {#if logStore.loading}
                    <Loader2 class="size-3.5 animate-spin" />
                {:else}
                    <RefreshCw class="size-3.5" />
                {/if}
                Refresh
            </Button>
            <Button variant="outline" size="sm" onclick={handleCopy}>
                <Clipboard class="size-3.5" />
                {copied ? "Copied" : "Copy"}
            </Button>
            <Button
                variant="outline"
                size="sm"
                onclick={() => logStore.clear()}
            >
                <Trash2 class="size-3.5" />
                Clear
            </Button>
        </div>
    </div>

    <div class="flex gap-2 mb-3">
        <div class="relative flex-1">
            <Search
                class="absolute left-2.5 top-1/2 -translate-y-1/2 size-3.5 text-muted-foreground pointer-events-none"
            />
            <Input
                placeholder="Search logs..."
                value={logStore.search}
                oninput={(e: Event) => {
                    logStore.search = (
                        e.currentTarget as HTMLInputElement
                    ).value;
                }}
                class="pl-8"
            />
        </div>
        <Select.Root
            type="single"
            value={logStore.levelFilter}
            onValueChange={(v) => {
                if (v) logStore.levelFilter = v as LevelFilter;
            }}
        >
            <Select.Trigger class="w-[120px]">
                {levelLabels[logStore.levelFilter]}
            </Select.Trigger>
            <Select.Content>
                <Select.Item value="all">All levels</Select.Item>
                <Select.Item value="info">Info</Select.Item>
                <Select.Item value="debug">Debug</Select.Item>
                <Select.Item value="warn">Warn</Select.Item>
                <Select.Item value="error">Error</Select.Item>
            </Select.Content>
        </Select.Root>
    </div>

    <div
        bind:this={scrollContainer}
        class="flex-1 rounded-lg border border-border bg-card overflow-y-auto"
    >
        {#if logStore.loading}
            <div
                class="flex flex-col items-center justify-center py-16 text-muted-foreground"
            >
                <Loader2 class="size-10 mb-3 opacity-30 animate-spin" />
                <p class="text-sm">Loading logs...</p>
            </div>
        {:else if logStore.filtered.length > 0}
            {#each logStore.filtered as entry, i (i)}
                <LogEntry {entry} />
            {/each}
        {:else if logStore.error}
            <div
                class="flex flex-col items-center justify-center py-16 text-muted-foreground"
            >
                <CircleAlert class="size-10 mb-3 opacity-30 text-destructive" />
                <p class="text-sm">Failed to load logs</p>
                <p
                    class="text-xs mt-1 font-mono text-destructive/70 max-w-md text-center"
                >
                    {logStore.error}
                </p>
            </div>
        {:else}
            <div
                class="flex flex-col items-center justify-center py-16 text-muted-foreground"
            >
                <ScrollText class="size-10 mb-3 opacity-30" />
                <p class="text-sm">No log entries</p>
                <p class="text-xs mt-1">
                    {#if logStore.search || logStore.levelFilter !== "all"}
                        Try adjusting your filters
                    {:else}
                        Logs will appear here during runtime
                    {/if}
                </p>
            </div>
        {/if}
    </div>
</div>
