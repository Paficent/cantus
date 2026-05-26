export type Tab = "mods" | "browse" | "settings" | "logs";

class NavStore {
  active = $state<Tab>("mods");
}

export const navStore = new NavStore();
