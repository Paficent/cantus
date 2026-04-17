<script lang="ts">
    import Input from "$lib/components/ui/input/input.svelte";
    import { Skeleton } from "$lib/components/ui/skeleton";
    import * as Select from "$lib/components/ui/select";
    import { BrowseCard } from "$lib/components/browse";
    import { browseStore } from "$lib/stores/browse.svelte";
    import { Search, Globe } from "lucide-svelte";
    import { toast } from "svelte-sonner";
    import type { CategoryFilter, SortOption } from "$lib/types/browse";
    import type { InstallResult } from "$lib/types/mod";

    const sortLabels: Record<SortOption, string> = {
        recent: "Recent",
        newest: "Newest",
        updated: "Updated",
        popular: "Popular",
        downloads: "Downloads",
        likes: "Most liked",
    };

    let sentinel = $state<HTMLElement | null>(null);
    let debounceTimer: ReturnType<typeof setTimeout>;

    function handleSearch(e: Event) {
        const value = (e.currentTarget as HTMLInputElement).value;
        clearTimeout(debounceTimer);
        debounceTimer = setTimeout(() => browseStore.setSearch(value), 300);
    }

    function showInstallToast(result: InstallResult) {
        if (result.installed.length === result.total) {
            const names = result.installed.join(", ");
            toast.success(
                result.total === 1
                    ? `Installed ${names}`
                    : `Installed ${result.total} mods`,
                result.total > 1 ? { description: names } : undefined,
            );
        } else if (result.installed.length > 0) {
            toast.warning(
                `Installed ${result.installed.length} of ${result.total} mods`,
                {
                    description: result.error ?? undefined,
                },
            );
        } else {
            toast.error("Installation failed", {
                description: result.error ?? undefined,
            });
        }
    }

    async function handleInstall(id: number) {
        try {
            const result = await browseStore.install(id);
            if (result) showInstallToast(result);
        } catch (e) {
            toast.error("Installation failed", { description: String(e) });
        }
    }

    $effect(() => {
        if (browseStore.categories.length === 0) {
            browseStore.loadCategories();
        }
    });

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
            <h2 class="text-base font-medium">Browse Mods</h2>
            <p class="text-xs text-muted-foreground mt-0.5">
                Created using GameBanana's API
            </p>
        </div>
    </div>

    <div class="flex gap-2 mb-3">
        <div class="relative flex-1">
            <Search
                class="absolute left-2.5 top-1/2 -translate-y-1/2 size-3.5 text-muted-foreground pointer-events-none"
            />
            <Input
                placeholder="Search GameBanana..."
                oninput={handleSearch}
                class="pl-8"
            />
        </div>
        <Select.Root
            type="single"
            value={String(browseStore.categoryFilter)}
            onValueChange={(v) => {
                if (v) browseStore.setCategory(v === "all" ? "all" : Number(v));
            }}
        >
            <Select.Trigger class="w-[140px]">
                {browseStore.categoryFilter === "all"
                    ? "All categories"
                    : (browseStore.categories.find(
                          (c) => c.id === browseStore.categoryFilter,
                      )?.name ?? "Category")}
            </Select.Trigger>
            <Select.Content>
                <Select.Item value="all">All categories</Select.Item>
                {#each browseStore.categories as cat}
                    <Select.Item value={String(cat.id)}>{cat.name}</Select.Item>
                {/each}
            </Select.Content>
        </Select.Root>
        <Select.Root
            type="single"
            value={browseStore.sort}
            onValueChange={(v) => {
                if (v) browseStore.setSort(v as SortOption);
            }}
        >
            <Select.Trigger class="w-[130px]">
                {sortLabels[browseStore.sort]}
            </Select.Trigger>
            <Select.Content>
                <Select.Item value="recent">Recent</Select.Item>
                <Select.Item value="newest">Newest</Select.Item>
                <Select.Item value="updated">Updated</Select.Item>
                <Select.Item value="popular">Popular</Select.Item>
                <Select.Item value="downloads">Downloads</Select.Item>
                <Select.Item value="likes">Most liked</Select.Item>
            </Select.Content>
        </Select.Root>
    </div>

    <div class="flex-1 overflow-y-auto pr-1">
        {#if browseStore.items.length > 0}
            <div class="grid grid-cols-3 gap-3">
                {#each browseStore.items as mod (mod.id)}
                    <BrowseCard
                        {mod}
                        installing={browseStore.installing.has(mod.id)}
                        oninstall={handleInstall}
                    />
                {/each}

                {#if browseStore.loading}
                    {#each Array(3) as _}
                        <div class="rounded-xl border overflow-hidden">
                            <Skeleton
                                class="aspect-video w-full rounded-none"
                            />
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
            <div
                class="flex flex-col items-center justify-center py-16 text-muted-foreground"
            >
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
