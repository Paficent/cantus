<script lang="ts">
  import Input from "$lib/components/ui/input/input.svelte";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { BrowseCard } from "$lib/components/browse";
  import { browseStore } from "$lib/stores/browse.svelte";
  import { Search, Globe } from "lucide-svelte";
  import type { CategoryFilter, SortOption } from "$lib/types/browse";

  let sentinel = $state<HTMLElement | null>(null);
  let debounceTimer: ReturnType<typeof setTimeout>;

  function handleSearch(e: Event) {
    const value = (e.currentTarget as HTMLInputElement).value;
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => browseStore.setSearch(value), 300);
  }

  $effect(() => {
    if (browseStore.items.length === 0 && browseStore.hasMore) {
      browseStore.loadMore();
    }
  });

  $effect(() => {
    if (!sentinel) return;
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) browseStore.loadMore();
      },
      { rootMargin: "200px" },
    );
    observer.observe(sentinel);
    return () => observer.disconnect();
  });
</script>

<div class="flex flex-col h-full">
  <div class="flex items-start justify-between mb-4">
    <div>
      <h2 class="text-base font-medium">Browse mods</h2>
      <p class="text-xs text-muted-foreground mt-0.5">
        {browseStore.totalLoaded} mods loaded
      </p>
    </div>
  </div>

  <div class="flex gap-2 mb-3">
    <div class="relative flex-1">
      <Search class="absolute left-2.5 top-1/2 -translate-y-1/2 size-3.5 text-muted-foreground pointer-events-none" />
      <Input
        placeholder="Search GameBanana..."
        oninput={handleSearch}
        class="pl-8"
      />
    </div>
    <select
      class="flex h-9 w-[140px] rounded-md border border-input bg-background px-3 py-1 text-sm text-muted-foreground cursor-pointer focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
      value={browseStore.categoryFilter}
      onchange={(e) => browseStore.setCategory((e.currentTarget as HTMLSelectElement).value as CategoryFilter)}
    >
      <option value="all">All categories</option>
      {#each browseStore.categories as cat}
        <option value={cat}>{cat}</option>
      {/each}
    </select>
    <select
      class="flex h-9 w-[130px] rounded-md border border-input bg-background px-3 py-1 text-sm text-muted-foreground cursor-pointer focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
      value={browseStore.sort}
      onchange={(e) => browseStore.setSort((e.currentTarget as HTMLSelectElement).value as SortOption)}
    >
      <option value="popular">Popular</option>
      <option value="recent">Recent</option>
      <option value="downloads">Downloads</option>
      <option value="likes">Most liked</option>
    </select>
  </div>

  <div class="flex-1 overflow-y-auto pr-1">
    {#if browseStore.items.length > 0}
      <div class="grid grid-cols-3 gap-3">
        {#each browseStore.items as mod (mod.id)}
          <BrowseCard
            {mod}
            installing={browseStore.installing.has(mod.id)}
            oninstall={(id) => browseStore.install(id)}
          />
        {/each}

        {#if browseStore.loading}
          {#each Array(3) as _}
            <div class="rounded-xl border overflow-hidden">
              <Skeleton class="aspect-video w-full rounded-none" />
              <div class="px-4 py-3">
                <Skeleton class="h-4 w-3/4 mb-2" />
                <Skeleton class="h-3 w-1/2 mb-3" />
                <Skeleton class="h-3 w-2/3" />
              </div>
              <div class="px-4 pb-3">
                <Skeleton class="h-7 w-full rounded-md" />
              </div>
            </div>
          {/each}
        {/if}
      </div>

      {#if browseStore.hasMore && !browseStore.loading}
        <div bind:this={sentinel} class="h-1"></div>
      {/if}

    {:else if browseStore.loading}
      <div class="grid grid-cols-3 gap-3">
        {#each Array(6) as _}
          <div class="rounded-xl border overflow-hidden">
            <Skeleton class="aspect-video w-full rounded-none" />
            <div class="px-4 py-3">
              <Skeleton class="h-4 w-3/4 mb-2" />
              <Skeleton class="h-3 w-1/2 mb-3" />
              <Skeleton class="h-3 w-2/3" />
            </div>
            <div class="px-4 pb-3">
              <Skeleton class="h-7 w-full rounded-md" />
            </div>
          </div>
        {/each}
      </div>

    {:else}
      <div class="flex flex-col items-center justify-center py-16 text-muted-foreground">
        <Globe class="size-10 mb-3 opacity-30" />
        <p class="text-sm">No mods found</p>
        <p class="text-xs mt-1">
          {#if browseStore.search || browseStore.categoryFilter !== "all"}
            Try adjusting your filters
          {:else}
            GameBanana mods will appear here
          {/if}
        </p>
      </div>
    {/if}
  </div>
</div>
