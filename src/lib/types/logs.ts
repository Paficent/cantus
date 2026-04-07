export type LogLevel = "info" | "debug" | "error" | "warn";

export interface LogEntry {
  timestamp: string;
  level: LogLevel;
  source: string;
  message: string;
  raw: string;
}

export type LevelFilter = LogLevel | "all";
