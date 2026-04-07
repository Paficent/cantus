import { MOCK_MODS } from "$lib/mock/data";
import type { Mod, TypeFilter, StatusFilter } from "$lib/types/mod";

class ModStore {
  mods = $state<Mod[]>(structuredClone(MOCK_MODS));
  search = $state("");
  typeFilter = $state<TypeFilter>("all");
  statusFilter = $state<StatusFilter>("all");

  readonly filtered = $derived(
    this.mods.filter((mod) => {
      const q = this.search.toLowerCase();
      const matchesSearch =
        !q ||
        mod.name.toLowerCase().includes(q) ||
        mod.id.includes(q) ||
        mod.author.toLowerCase().includes(q);
      const matchesType = this.typeFilter === "all" || mod.type === this.typeFilter;
      const matchesStatus =
        this.statusFilter === "all" ||
        (this.statusFilter === "enabled" ? mod.enabled : !mod.enabled);
      return matchesSearch && matchesType && matchesStatus;
    }),
  );

  readonly enabledCount = $derived(this.mods.filter((m) => m.enabled).length);
  readonly totalCount = $derived(this.mods.length);

  toggle(id: string) {
    const mod = this.mods.find((m) => m.id === id);
    if (mod) mod.enabled = !mod.enabled;
  }

  remove(id: string) {
    this.mods = this.mods.filter((m) => m.id !== id);
  }

  getById(id: string): Mod | undefined {
    return this.mods.find((m) => m.id === id);
  }
}

export const modStore = new ModStore();
