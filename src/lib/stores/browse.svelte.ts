import { invoke } from "@tauri-apps/api/core";
import { settingsStore } from "$lib/stores/settings.svelte";
import { modStore } from "$lib/stores/mods.svelte";
import type {
    BrowseMod,
    BrowsePage,
    CategoryFilter,
    CategoryInfo,
    SortOption,
} from "$lib/types/browse";
import type { InstallResult } from "$lib/types/mod";

const PER_PAGE = 15;

class BrowseStore {
    items = $state<BrowseMod[]>([]);
    page = $state(1);
    loading = $state(false);
    hasMore = $state(true);
    search = $state("");
    categoryFilter = $state<CategoryFilter>("all");
    sort = $state<SortOption>("recent");
    categories = $state<CategoryInfo[]>([]);
    installing = $state<Set<number>>(new Set());

    async loadCategories() {
        this.categories = await invoke<CategoryInfo[]>("browse_categories");
    }

    async loadMore() {
        if (this.loading || !this.hasMore) return;
        this.loading = true;

        try {
            const result = await invoke<BrowsePage>("browse_mods", {
                page: this.page,
                perPage: PER_PAGE,
                sort: this.sort,
                search: this.search,
                categoryId: this.categoryFilter === "all" ? null : this.categoryFilter,
                showNsfw: settingsStore.cantus.show_nsfw,
            });

            this.items = [...this.items, ...result.mods];
            this.hasMore = result.has_more;
            this.page += 1;
        } catch (e) {
            console.error(e);
            this.hasMore = false;
        }
        this.loading = false;
    }

    reset() {
        this.items = [];
        this.page = 1;
        this.hasMore = true;
        this.loadMore();
    }

    setSearch(value: string) {
        this.search = value;
        this.reset();
    }

    setCategory(value: CategoryFilter) {
        this.categoryFilter = value;
        this.reset();
    }

    setSort(value: SortOption) {
        this.sort = value;
        this.reset();
    }

    async install(id: number): Promise<InstallResult | null> {
        if (this.installing.has(id)) return null;

        const mod = this.items.find((m) => m.id === id);
        if (!mod) return null;

        this.installing = new Set([...this.installing, id]);

        try {
            const result = await invoke<InstallResult>("browse_install_mod", {
                modId: id,
                modName: mod.name,
                modAuthor: mod.author,
            });
            await modStore.load();
            return result;
        } finally {
            const next = new Set(this.installing);
            next.delete(id);
            this.installing = next;
        }
    }
}

export const browseStore = new BrowseStore();
