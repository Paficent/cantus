export interface BrowseMod {
  id: number;
  name: string;
  author: string;
  views: number;
  likes: number;
  downloads: number;
  category: string;
  screenshot: string;
  date_added: number;
  date_updated: number;
}

export interface BrowsePage {
  mods: BrowseMod[];
  has_more: boolean;
}

export interface CategoryInfo {
  id: number;
  name: string;
}

export type CategoryFilter = "all" | number;
export type SortOption =
  | "recent"
  | "newest"
  | "updated"
  | "popular"
  | "downloads"
  | "likes";
