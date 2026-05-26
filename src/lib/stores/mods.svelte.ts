import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type {
  Mod,
  InstallResult,
  TypeFilter,
  StatusFilter,
} from "$lib/types/mod";

class ModStore {
  mods = $state<Mod[]>([]);
  search = $state("");
  typeFilter = $state<TypeFilter>("all");
  statusFilter = $state<StatusFilter>("all");
  loading = $state(false);
  installing = $state(false);
  watching = false;

  filtered = $derived(
    this.mods.filter((mod) => {
      const q = this.search.toLowerCase();
      if (q) {
        if (
          !mod.name.toLowerCase().includes(q) &&
          !mod.id.includes(q) &&
          !mod.author.toLowerCase().includes(q)
        ) {
          return false;
        }
      }
      if (this.typeFilter !== "all" && mod.type !== this.typeFilter)
        return false;
      if (this.statusFilter === "enabled" && !mod.enabled) return false;
      if (this.statusFilter === "disabled" && mod.enabled) return false;
      return true;
    }),
  );

  enabledCount = $derived(this.mods.filter((m) => m.enabled).length);
  totalCount = $derived(this.mods.length);

  async load() {
    this.loading = true;
    try {
      this.mods = await invoke<Mod[]>("list_mods");
    } catch (e) {
      console.error(e);
      this.mods = [];
    }
    this.loading = false;
  }

  async startWatching() {
    if (this.watching) return;
    this.watching = true;
    await invoke("watch_mods_folder");
    await listen("mods-changed", () => this.load());
  }

  async toggle(id: string) {
    const newState = await invoke<boolean>("toggle_mod", { id });
    const mod = this.mods.find((m) => m.id === id);
    if (mod) mod.enabled = newState;
  }

  async remove(id: string) {
    await invoke("remove_mod", { id });
    this.mods = this.mods.filter((m) => m.id !== id);
  }

  async openModFolder(id: string) {
    await invoke("open_mod_folder", { id });
  }

  async install(): Promise<InstallResult | null> {
    this.installing = true;
    try {
      const result = await invoke<InstallResult | null>("install_mod");
      if (result) await this.load();
      return result;
    } finally {
      this.installing = false;
    }
  }

  getById(id: string) {
    return this.mods.find((m) => m.id === id);
  }
}

export const modStore = new ModStore();
