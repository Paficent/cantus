import type { BrowseMod, CategoryFilter, SortOption } from "$lib/types/browse";
import { getMockPage, MOCK_CATEGORIES } from "$lib/mock/browse";

class BrowseStore {
  items = $state<BrowseMod[]>([]);
  page = $state(0);
  loading = $state(false);
  hasMore = $state(true);
  search = $state("");
  categoryFilter = $state<CategoryFilter>("all");
  sort = $state<SortOption>("recent");
  categories = $state<string[]>(MOCK_CATEGORIES);
  installing = $state<Set<number>>(new Set());

  readonly totalLoaded = $derived(this.items.length);

  async loadMore() {
    if (this.loading || !this.hasMore) return;
    this.loading = true;

    await new Promise((r) => setTimeout(r, 400));

    const result = getMockPage(
      this.page,
      this.search,
      this.categoryFilter,
      this.sort,
    );

    this.items = [...this.items, ...result.items];
    this.hasMore = result.hasMore;
    this.page += 1;
    this.loading = false;
  }

  reset() {
    this.items = [];
    this.page = 0;
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

  async install(id: number) {
    if (this.installing.has(id)) return;
    this.installing = new Set([...this.installing, id]);

    await new Promise((r) => setTimeout(r, 1200));

    this.installing = new Set([...this.installing].filter((i) => i !== id));
  }
}

export const browseStore = new BrowseStore();
