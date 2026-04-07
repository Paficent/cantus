<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import { LogEntry } from "$lib/components/logs";
  import { logStore } from "$lib/stores/logs.svelte";
  import { Search, Trash2, ScrollText } from "lucide-svelte";
  import type { LevelFilter } from "$lib/types/log";
</script>

<div class="flex flex-col h-full">
  <div class="flex items-start justify-between mb-4">
    <div>
      <h2 class="text-base font-medium">Runtime logs</h2>
      <p class="text-xs text-muted-foreground mt-0.5">
        {logStore.filtered.length} of {logStore.counts.total} entries
        <span class="ml-2 text-muted-foreground/50">
          {logStore.counts.info} info · {logStore.counts.debug} debug · {logStore.counts.warn} warn · {logStore.counts.error} error
        </span>
      </p>
    </div>
    <Button variant="outline" size="sm" onclick={() => logStore.clear()}>
      <Trash2 class="size-3.5" />
      Clear
    </Button>
  </div>

  <div class="flex gap-2 mb-3">
    <div class="relative flex-1">
      <Search class="absolute left-2.5 top-1/2 -translate-y-1/2 size-3.5 text-muted-foreground pointer-events-none" />
      <Input
        placeholder="Search logs..."
        value={logStore.search}
        oninput={(e: Event) => {
          logStore.search = (e.currentTarget as HTMLInputElement).value;
        }}
        class="pl-8"
      />
    </div>
    <select
      class="flex h-9 w-[120px] rounded-md border border-input bg-background px-3 py-1 text-sm text-muted-foreground cursor-pointer focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
      value={logStore.levelFilter}
      onchange={(e) => { logStore.levelFilter = (e.currentTarget as HTMLSelectElement).value as LevelFilter; }}
    >
      <option value="all">All levels</option>
      <option value="info">Info</option>
      <option value="debug">Debug</option>
      <option value="warn">Warn</option>
      <option value="error">Error</option>
    </select>
    <select
      class="flex h-9 w-[140px] rounded-md border border-input bg-background px-3 py-1 text-sm text-muted-foreground cursor-pointer focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
      value={logStore.sourceFilter}
      onchange={(e) => { logStore.sourceFilter = (e.currentTarget as HTMLSelectElement).value; }}
    >
      <option value="all">All sources</option>
      {#each logStore.sources as source}
        <option value={source}>{source}</option>
      {/each}
    </select>
  </div>

  <div class="flex-1 rounded-lg border border-border bg-card overflow-y-auto">
    {#if logStore.filtered.length > 0}
      {#each logStore.filtered as entry, i (i)}
        <LogEntry {entry} />
      {/each}
    {:else}
      <div class="flex flex-col items-center justify-center py-16 text-muted-foreground">
        <ScrollText class="size-10 mb-3 opacity-30" />
        <p class="text-sm">No log entries</p>
        <p class="text-xs mt-1">
          {#if logStore.search || logStore.levelFilter !== "all" || logStore.sourceFilter !== "all"}
            Try adjusting your filters
          {:else}
            Logs will appear here during runtime
          {/if}
        </p>
      </div>
    {/if}
  </div>
</div>
