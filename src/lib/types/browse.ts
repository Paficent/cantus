export interface BrowseMod {
  id: number;
  name: string;
  author: string;
  views: number;
  likes: number;
  downloads: number;
  category: string;
  screenshot: string;
}

export type CategoryFilter = "all" | string;
export type SortOption = "popular" | "recent" | "downloads" | "likes";
