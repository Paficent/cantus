export type ModType = "native" | "lua" | "asset";

export type TypeFilter = ModType | "all";
export type StatusFilter = "all" | "enabled" | "disabled";

export interface Mod {
    id: string;
    name: string;
    version: string;
    author: string;
    type: ModType;
    enabled: boolean;
}

export interface InstallResult {
    installed: string[];
    total: number;
    error: string | null;
}
