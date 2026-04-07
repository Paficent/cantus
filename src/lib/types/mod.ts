export type ModType = "native" | "lua" | "asset";

export interface Mod {
  id: string;
  name: string;
  version: string;
  author: string;
  type: ModType;
  enabled: boolean;
}

export type TypeFilter = ModType | "all";
export type StatusFilter = "all" | "enabled" | "disabled";
