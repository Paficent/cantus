export interface CantusSettings {
  game_directory: string | null;
  onboarding_complete: boolean;
  show_nsfw: boolean;
}

export interface JeodeSettings {
  last_update_check: number;
  overlays_enabled: boolean;
  debug: boolean;
  allow_unsafe_functions: boolean;
  suppress_native_warnings: boolean;
  toggle_key: string;
}
