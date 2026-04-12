import type { LogEntry, LogLevel } from "$lib/types/logs";

const LOG_PATTERN = /^\[([^\]]+)\]\s+\[(\w+)\]\s+\[([^\]]+)\]\s+(.+)/;

function normalizeLevel(raw: string): LogLevel {
  const lower = raw.toLowerCase();
  if (lower === "warning") return "warn";
  return lower as LogLevel;
}

export function parseLine(line: string): LogEntry | null {
  const trimmed = line.replace(/\r$/, "");
  const match = trimmed.match(LOG_PATTERN);
  if (!match) return null;
  return {
    timestamp: match[1],
    level: normalizeLevel(match[2]),
    source: match[3],
    message: match[4],
    raw: trimmed,
  };
}

export function parseLog(text: string): LogEntry[] {
  const cleaned = text.replace(/^\uFEFF/, "");
  return cleaned
    .split(/\r?\n/)
    .filter((l) => l.length > 0)
    .map(parseLine)
    .filter((e): e is LogEntry => e !== null);
}
