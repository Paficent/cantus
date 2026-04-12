import type { LogEntry, LogLevel } from "$lib/types/logs";

// Regex Sucks
const LOG_PATTERN = /^\[([^\]]+)\]\s+\[(\w+)\]\s+\[([^\]]+)\]\s+(.*)$/;

export function parseLine(line: string): LogEntry | null {
  const match = line.match(LOG_PATTERN);
  if (!match) return null;
  return {
    timestamp: match[1],
    level: match[2] as LogLevel,
    source: match[3],
    message: match[4],
    raw: line,
  };
}

export function parseLog(text: string): LogEntry[] {
  return text
    .split("\n")
    .filter((l) => l.trim())
    .map(parseLine)
    .filter((e): e is LogEntry => e !== null);
}
