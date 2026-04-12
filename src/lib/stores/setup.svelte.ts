import { invoke } from "@tauri-apps/api/core";
import type { OnboardingStep, JeodeStatus } from "$lib/types/setup";

interface Settings {
  game_directory: string | null;
  onboarding_complete: boolean;
  theme?: string;
}

function applyTheme(theme: string) {
  document.documentElement.classList.toggle("dark", theme !== "light");
}

class SetupStore {
  step = $state<OnboardingStep>("directory");
  gameDirectory = $state("");
  directoryValid = $state(false);
  validating = $state(false);
  jeodeStatus = $state<JeodeStatus>("unknown");
  complete = $state(false);
  error = $state("");
  loaded = $state(false);

  async loadSettings() {
    try {
      const settings = await invoke<Settings>("load_settings");
      applyTheme(settings.theme ?? "dark");

      if (settings.onboarding_complete && settings.game_directory) {
        this.gameDirectory = settings.game_directory;
        this.directoryValid = true;
        this.complete = true;
      } else if (settings.game_directory) {
        this.gameDirectory = settings.game_directory;
      }
    } catch (e) {
      console.error("Failed to load settings:", e);
    } finally {
      this.loaded = true;
    }
  }

  private async persistSettings() {
    try {
      await invoke("save_settings", {
        settings: {
          game_directory: this.gameDirectory || null,
          onboarding_complete: this.complete,
          show_nsfw: false,
          theme: "dark",
        },
      });
    } catch (e) {
      console.error("Failed to save settings:", e);
    }
  }

  async browseGameDirectory() {
    this.error = "";

    try {
      const selected = await invoke<string | null>("select_game_directory");
      if (selected) {
        this.gameDirectory = selected;
        await this.validateDirectory();
      }
    } catch (e) {
      this.error = String(e);
    }
  }

  async validateDirectory() {
    if (!this.gameDirectory) {
      this.directoryValid = false;
      return;
    }

    this.validating = true;
    this.error = "";

    try {
      const valid = await invoke<boolean>("validate_game_directory", {
        path: this.gameDirectory,
      });
      this.directoryValid = valid;

      if (!valid) {
        this.error = "Could not find a valid game installation at this path.";
      }
    } catch (e) {
      this.directoryValid = false;
      this.error = String(e);
    } finally {
      this.validating = false;
    }
  }

  confirmDirectory() {
    if (!this.directoryValid) return;
    this.step = "jeode";
    this.checkJeode();
  }

  async checkJeode() {
    this.jeodeStatus = "checking";
    this.error = "";

    try {
      const installed = await invoke<boolean>("check_jeode_installed", {
        gameDir: this.gameDirectory,
      });
      this.jeodeStatus = installed ? "installed" : "not_installed";
    } catch (e) {
      this.jeodeStatus = "not_installed";
      this.error = String(e);
    }
  }

  async installJeode() {
    this.jeodeStatus = "installing";
    this.error = "";

    try {
      await invoke("install_jeode", { gameDir: this.gameDirectory });
      this.jeodeStatus = "installed";
    } catch (e) {
      this.jeodeStatus = "install_failed";
      this.error = String(e);
    }
  }

  async finishOnboarding() {
    this.complete = true;
    await this.persistSettings();
  }

  goBack() {
    if (this.step === "jeode") {
      this.step = "directory";
      this.jeodeStatus = "unknown";
    }
  }
}

export const setupStore = new SetupStore();
