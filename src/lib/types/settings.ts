export interface CantusSettings {
  game_directory: string | null;
  onboarding_complete: boolean;
  show_nsfw: boolean;
  convert_images: boolean;
  theme: "dark" | "light";
}

export interface JeodeSettings {
  last_update_check: number;
  ui_visible_on_startup: boolean;
  debug: boolean;
  allow_unsafe_functions: boolean;
  suppress_native_warnings: boolean;
  toggle_key: string;
}

export const TOGGLE_KEYS = [
  "F1",
  "F2",
  "F3",
  "F4",
  "F5",
  "F6",
  "F7",
  "F8",
  "F9",
  "F10",
  "F11",
  "F12",
  "Insert",
  "Delete",
  "Home",
  "End",
  "PageUp",
  "PageDown",
  "Pause",
  "ScrollLock",
  "NumLock",
  "Tab",
  "CapsLock",
  "Backspace",
  "Tilde",
  "Minus",
  "Equals",
  "LeftBracket",
  "RightBracket",
  "Backslash",
  "Semicolon",
  "Quote",
  "Comma",
  "Period",
  "Slash",
  "Numpad0",
  "Numpad1",
  "Numpad2",
  "Numpad3",
  "Numpad4",
  "Numpad5",
  "Numpad6",
  "Numpad7",
  "Numpad8",
  "Numpad9",
  "NumpadMultiply",
  "NumpadAdd",
  "NumpadSubtract",
  "NumpadDecimal",
  "NumpadDivide",
] as const;

const SPECIAL: Record<string, string> = {
  Backquote: "Tilde",
  Equal: "Equals",
  BracketLeft: "LeftBracket",
  BracketRight: "RightBracket",
};

export const CODE_TO_JEODE: Record<string, string> = (() => {
  const out: Record<string, string> = { ...SPECIAL };
  for (const k of TOGGLE_KEYS) {
    if (!Object.values(out).includes(k)) out[k] = k;
  }
  return out;
})();
