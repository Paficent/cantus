import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Mod, TypeFilter, StatusFilter } from "$lib/types/mod";

class ModStore {
  mods = $state<Mod[]>([]);
  search = $state("");
  typeFilter = $state<TypeFilter>("all");
  statusFilter = $state<StatusFilter>("all");
  loading = $state(false);
  installing = $state(false);
  private watching = false;

  readonly filtered = $derived(
    this.mods.filter((mod) => {
      const q = this.search.toLowerCase();
      const matchesSearch =
        !q ||
        mod.name.toLowerCase().includes(q) ||
        mod.id.includes(q) ||
        mod.author.toLowerCase().includes(q);
      const matchesType =
        this.typeFilter === "all" || mod.type === this.typeFilter;
      const matchesStatus =
        this.statusFilter === "all" ||
        (this.statusFilter === "enabled" ? mod.enabled : !mod.enabled);
      return matchesSearch && matchesType && matchesStatus;
    }),
  );

  readonly enabledCount = $derived(this.mods.filter((m) => m.enabled).length);
  readonly totalCount = $derived(this.mods.length);

  async load() {
    this.loading = true;
    try {
      this.mods = await invoke<Mod[]>("list_mods");
    } catch (e) {
      console.error("Failed to load mods:", e);
      this.mods = [];
    } finally {
      this.loading = false;
    }
  }

  async startWatching() {
    if (this.watching) return;
    this.watching = true;
    try {
      await invoke("watch_mods_folder");
      await listen("mods-changed", () => {
        this.load();
      });
    } catch (e) {
      console.error("Failed to start mods watcher:", e);
    }
  }

  async toggle(id: string) {
    try {
      const newState = await invoke<boolean>("toggle_mod", { id });
      const mod = this.mods.find((m) => m.id === id);
      if (mod) mod.enabled = newState;
    } catch (e) {
      console.error("Failed to toggle mod:", e);
    }
  }

  async remove(id: string) {
    try {
      await invoke("remove_mod", { id });
      this.mods = this.mods.filter((m) => m.id !== id);
    } catch (e) {
      console.error("Failed to remove mod:", e);
    }
  }

  async openModFolder(id: string) {
    try {
      await invoke("open_mod_folder", { id });
    } catch (e) {
      console.error("Failed to open mod folder:", e);
    }
  }

  async install(): Promise<string | null> {
    this.installing = true;
    try {
      const name = await invoke<string | null>("install_mod");
      if (name) await this.load();
      return name;
    } catch (e) {
      console.error("Failed to install mod:", e);
      throw e;
    } finally {
      this.installing = false;
    }
  }

  getById(id: string): Mod | undefined {
    return this.mods.find((m) => m.id === id);
  }
}

export const modStore = new ModStore();
