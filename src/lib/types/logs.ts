export type LogLevel = "info" | "debug" | "warn" | "error";
export type LevelFilter = LogLevel | "all";

export interface LogEntry {
  timestamp: string;
  level: LogLevel;
  source: string;
  message: string;
  raw: string;
}
