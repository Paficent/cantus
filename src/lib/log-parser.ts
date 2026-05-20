import type { LogEntry, LogLevel } from "$lib/types/logs";

export function parseLine(line: string): LogEntry | null {
    const trimmed = line.replace(/\r$/, "");
    const m = trimmed.match(/^\[([^\]]+)\]\s+\[(\w+)\]\s+\[([^\]]+)\]\s+(.+)/);
    if (!m) return null;

    let level = m[2].toLowerCase();
    if (level === "warning") level = "warn";

    return {
        timestamp: m[1],
        level: level as LogLevel,
        source: m[3],
        message: m[4],
        raw: trimmed,
    };
}

export function parseLog(text: string): LogEntry[] {
    return text
        .replace(/^\uFEFF/, "")
        .split(/\r?\n/)
        .filter((l) => l.length > 0)
        .map(parseLine)
        .filter((e): e is LogEntry => e !== null);
}
