import { MOCK_LOGS } from "$lib/mock/logs";
import type { LogEntry, LevelFilter } from "$lib/types/log";

class LogStore {
  entries = $state<LogEntry[]>(MOCK_LOGS);
  search = $state("");
  levelFilter = $state<LevelFilter>("all");
  sourceFilter = $state("all");

  readonly sources = $derived(
    [...new Set(this.entries.map((e) => e.source))].sort(),
  );

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
      const matchesSource =
        this.sourceFilter === "all" || entry.source === this.sourceFilter;
      return matchesSearch && matchesLevel && matchesSource;
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

  clear() {
    this.entries = [];
    this.search = "";
    this.levelFilter = "all";
    this.sourceFilter = "all";
  }
}

export const logStore = new LogStore();
