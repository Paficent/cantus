import { invoke } from "@tauri-apps/api/core";
import type {
  OnboardingStep,
  JeodeStatus,
  ProtonStatus,
} from "$lib/types/setup";

function applyTheme(theme: string) {
  document.documentElement.classList.toggle("dark", theme !== "light");
}

class SetupStore {
  step = $state<OnboardingStep>("directory");
  gameDirectory = $state("");
  directoryValid = $state(false);
  validating = $state(false);
  jeodeStatus = $state<JeodeStatus>("unknown");
  protonNeeded = $state(false);
  protonStatus = $state<ProtonStatus>("idle");
  complete = $state(false);
  error = $state("");
  loaded = $state(false);

  async loadSettings() {
    try {
      const settings = await invoke<any>("load_settings");
      applyTheme(settings.theme ?? "dark");

      if (settings.onboarding_complete && settings.game_directory) {
        this.gameDirectory = settings.game_directory;
        this.directoryValid = true;
        this.complete = true;
      } else if (settings.game_directory) {
        this.gameDirectory = settings.game_directory;
      }
    } catch (e) {
      console.error(e);
    }
    this.loaded = true;
  }

  async saveProgress() {
    await invoke("save_settings", {
      settings: {
        game_directory: this.gameDirectory || null,
        onboarding_complete: this.complete,
        show_nsfw: false,
        theme: "dark",
      },
    });
  }

  async browseGameDirectory() {
    this.error = "";
    const selected = await invoke<string | null>("select_game_directory");
    if (selected) {
      this.gameDirectory = selected;
      await this.validateDirectory();
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
    }

    this.validating = false;
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
      if (installed) await this.refreshProton();
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
      await this.refreshProton();
    } catch (e) {
      this.jeodeStatus = "install_failed";
      this.error = String(e);
    }
  }

  async refreshProton() {
    try {
      this.protonNeeded = await invoke<boolean>("proton_step_needed", {
        gameDir: this.gameDirectory,
      });
    } catch (e) {
      this.protonNeeded = false;
      console.error(e);
    }
  }

  async continueFromJeode() {
    if (this.protonNeeded) {
      this.protonStatus = "idle";
      this.error = "";
      this.step = "proton";
    } else {
      await this.finishOnboarding();
    }
  }

  async applyProtonOverride() {
    this.protonStatus = "applying";
    this.error = "";
    try {
      await invoke("apply_proton_override", { gameDir: this.gameDirectory });
      this.protonStatus = "applied";
      await this.finishOnboarding();
    } catch (e) {
      this.protonStatus = "failed";
      this.error = String(e);
    }
  }

  async skipProton() {
    await this.finishOnboarding();
  }

  async finishOnboarding() {
    this.complete = true;
    await this.saveProgress();
  }

  goBack() {
    if (this.step === "jeode") {
      this.step = "directory";
      this.jeodeStatus = "unknown";
    } else if (this.step === "proton") {
      this.step = "jeode";
      this.protonStatus = "idle";
      this.error = "";
    }
  }
}

export const setupStore = new SetupStore();
