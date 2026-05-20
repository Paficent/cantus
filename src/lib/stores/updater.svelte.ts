import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { getVersion } from "@tauri-apps/api/app";
import { toast } from "svelte-sonner";

class UpdaterStore {
  currentVersion = $state("");
  pending = $state<Update | null>(null);
  checking = $state(false);
  installing = $state(false);
  downloaded = $state(0);
  totalBytes = $state(0);

  private initialized = false;

  progressPct = $derived(
    this.totalBytes > 0
      ? Math.round((this.downloaded / this.totalBytes) * 100)
      : 0,
  );

  async init() {
    if (this.initialized) return;
    this.initialized = true;
    try {
      this.currentVersion = await getVersion();
    } catch (e) {
      console.error("couldnt read app version: ", e);
    }
    await this.checkSilent();
  }

  async checkSilent() {
    if (this.checking) return;
    this.checking = true;
    try {
      const update = await check();
      this.pending = update ?? null;
    } catch (e) {
      console.warn("Update check failed: ", e);
    }
    this.checking = false;
  }

  async checkManual() {
    if (this.checking) return;
    this.checking = true;
    try {
      const update = await check();
      this.pending = update ?? null;
      if (update) {
        toast.success(`Update available: v${update.version}`);
      } else {
        toast.info("You are up to date");
      }
    } catch (e) {
      toast.error("Update check failed", { description: String(e) });
    }
    this.checking = false;
  }

  async install() {
    if (!this.pending || this.installing) return;
    this.installing = true;
    this.downloaded = 0;
    this.totalBytes = 0;
    try {
      await this.pending.downloadAndInstall((event) => {
        if (event.event === "Started") {
          this.totalBytes = event.data.contentLength ?? 0;
        } else if (event.event === "Progress") {
          this.downloaded += event.data.chunkLength;
        }
      });
      await relaunch();
    } catch (e) {
      this.installing = false;
      toast.error("Update failed", { description: String(e) });
    }
  }
}

export const updaterStore = new UpdaterStore();
