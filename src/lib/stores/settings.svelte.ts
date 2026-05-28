import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { CantusSettings, JeodeSettings } from "$lib/types/settings";

function applyTheme(theme: "dark" | "light") {
  document.documentElement.classList.toggle("dark", theme === "dark");
}

class SettingsStore {
  cantus = $state<CantusSettings>({
    game_directory: null,
    onboarding_complete: false,
    show_nsfw: false,
    convert_images: false,
    theme: "dark",
  });

  jeode = $state<JeodeSettings>({
    last_update_check: 0,
    overlays_enabled: true,
    debug: false,
    allow_unsafe_functions: false,
    suppress_native_warnings: false,
    toggle_key: "Tilde",
  });

  loading = $state(false);
  saving = $state(false);
  watching = false;

  async load() {
    this.loading = true;
    try {
      const cantus = await invoke<CantusSettings>("load_settings");
      const jeode = await invoke<JeodeSettings>("read_jeode_settings");
      this.cantus = cantus;
      this.jeode = jeode;
      applyTheme(cantus.theme ?? "dark");
    } catch (e) {
      console.error(e);
    }
    this.loading = false;
  }

  async startWatching() {
    if (this.watching) return;
    this.watching = true;
    await invoke("watch_log_file");
    await listen("jeode-settings-changed", async () => {
      this.jeode = await invoke<JeodeSettings>("read_jeode_settings");
    });
  }

  async saveCantus() {
    this.saving = true;
    try {
      await invoke("save_settings", { settings: this.cantus });
    } catch (e) {
      console.error(e);
    }
    this.saving = false;
  }

  async saveJeode() {
    this.saving = true;
    try {
      await invoke("write_jeode_settings", { jeodeSettings: this.jeode });
    } catch (e) {
      console.error(e);
    }
    this.saving = false;
  }

  async updateCantus<K extends keyof CantusSettings>(
    key: K,
    value: CantusSettings[K],
  ) {
    this.cantus = { ...this.cantus, [key]: value };
    if (key === "theme") applyTheme(value as "dark" | "light");
    await this.saveCantus();
  }

  async updateJeode<K extends keyof JeodeSettings>(
    key: K,
    value: JeodeSettings[K],
  ) {
    this.jeode = { ...this.jeode, [key]: value };
    await this.saveJeode();
  }
}

export const settingsStore = new SettingsStore();
