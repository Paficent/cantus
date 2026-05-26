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
  watching = false;

  filtered = $derived(
    this.entries.filter((e) => {
      if (this.levelFilter !== "all" && e.level !== this.levelFilter)
        return false;
      const q = this.search.toLowerCase();
      if (!q) return true;
      return (
        e.message.toLowerCase().includes(q) ||
        e.source.toLowerCase().includes(q) ||
        e.timestamp.includes(q)
      );
    }),
  );

  counts = $derived.by(() => {
    const c = {
      total: this.entries.length,
      shown: this.filtered.length,
      info: 0,
      debug: 0,
      warn: 0,
      error: 0,
    };
    for (const e of this.entries) c[e.level]++;
    return c;
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
    }
    this.loading = false;
  }

  async startWatching() {
    if (this.watching) return;
    this.watching = true;
    await invoke("watch_log_file");
    await listen("log-changed", () => this.load());
  }

  clear() {
    this.entries = [];
    this.search = "";
    this.levelFilter = "all";
    this.error = "";
  }

  copyToClipboard() {
    const text = this.filtered.map((e) => e.raw).join("\n");
    navigator.clipboard.writeText(text);
  }
}

export const logStore = new LogStore();
