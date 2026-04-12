export interface CantusSettings {
  game_directory: string | null;
  onboarding_complete: boolean;
  show_nsfw: boolean;
  theme: "dark" | "light";
}

export interface JeodeSettings {
  last_update_check: number;
  overlays_enabled: boolean;
  debug: boolean;
  allow_unsafe_functions: boolean;
  suppress_native_warnings: boolean;
  toggle_key: string;
}

export const TOGGLE_KEYS = [
  "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
  "Insert", "Delete", "Home", "End", "PageUp", "PageDown",
  "Pause", "ScrollLock", "NumLock", "Tab", "CapsLock", "Backspace",
  "Tilde", "Minus", "Equals", "LeftBracket", "RightBracket", "Backslash",
  "Semicolon", "Quote", "Comma", "Period", "Slash",
  "Numpad0", "Numpad1", "Numpad2", "Numpad3", "Numpad4",
  "Numpad5", "Numpad6", "Numpad7", "Numpad8", "Numpad9",
  "NumpadMultiply", "NumpadAdd", "NumpadSubtract", "NumpadDecimal", "NumpadDivide",
] as const;

export const CODE_TO_JEODE: Record<string, string> = {
  F1: "F1", F2: "F2", F3: "F3", F4: "F4", F5: "F5", F6: "F6",
  F7: "F7", F8: "F8", F9: "F9", F10: "F10", F11: "F11", F12: "F12",
  Insert: "Insert", Delete: "Delete", Home: "Home", End: "End",
  PageUp: "PageUp", PageDown: "PageDown", Pause: "Pause",
  ScrollLock: "ScrollLock", NumLock: "NumLock", Tab: "Tab",
  CapsLock: "CapsLock", Backspace: "Backspace",
  Backquote: "Tilde", Minus: "Minus", Equal: "Equals",
  BracketLeft: "LeftBracket", BracketRight: "RightBracket",
  Backslash: "Backslash", Semicolon: "Semicolon", Quote: "Quote",
  Comma: "Comma", Period: "Period", Slash: "Slash",
  Numpad0: "Numpad0", Numpad1: "Numpad1", Numpad2: "Numpad2",
  Numpad3: "Numpad3", Numpad4: "Numpad4", Numpad5: "Numpad5",
  Numpad6: "Numpad6", Numpad7: "Numpad7", Numpad8: "Numpad8",
  Numpad9: "Numpad9", NumpadMultiply: "NumpadMultiply",
  NumpadAdd: "NumpadAdd", NumpadSubtract: "NumpadSubtract",
  NumpadDecimal: "NumpadDecimal", NumpadDivide: "NumpadDivide",
};
