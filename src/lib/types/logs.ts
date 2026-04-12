import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { parseLog } from "$lib/log-parser";
import type { LogEntry, LevelFilter } from "$lib/types/logs";

class LogStore {
  entries = $state<LogEntry[]>([]);
  search = $state("");
  levelFilter = $state<LevelFilter>("all");
  loading = $state(false);
  error = $state("");
  private watching = false;

  readonly filtered = $derived(
    this.entries.filter((entry) => {
      const q = this.search.toLowerCase();
      const matchesSearch =
        !q ||
        entry.message.toLowerCase().includes(q) ||
        entry.source.toLowerCase().includes(q) ||
        entry.timestamp.includes(q);
      const matchesLevel =
        this.levelFilter === "all" || entry.level === this.levelFilter;
      return matchesSearch && matchesLevel;
    }),
  );

  readonly counts = $derived({
    total: this.entries.length,
    shown: this.filtered.length,
    info: this.entries.filter((e) => e.level === "info").length,
    debug: this.entries.filter((e) => e.level === "debug").length,
    warn: this.entries.filter((e) => e.level === "warn").length,
    error: this.entries.filter((e) => e.level === "error").length,
  });

  async load() {
    this.loading = true;
    this.error = "";
    try {
      const raw = await invoke<string>("read_log_file");
      this.entries = parseLog(raw);
    } catch (e) {
      this.error = String(e);
      this.entries = [];
    } finally {
      this.loading = false;
    }
  }

  async startWatching() {
    if (this.watching) return;
    this.watching = true;
    try {
      await invoke("watch_log_file");
      await listen("log-changed", () => {
        this.load();
      });
    } catch (e) {
      console.error("Failed to start log watcher:", e);
    }
  }

  clear() {
    this.entries = [];
    this.search = "";
    this.levelFilter = "all";
    this.error = "";
  }

  copyToClipboard() {
    const text = this.filtered.map((e) => e.raw).join("\n");
    navigator.clipboard.writeText(text).catch((e) => {
      console.error("Failed to copy logs:", e);
    });
  }
}

export const logStore = new LogStore();
