export type OnboardingStep = "directory" | "jeode" | "proton";

export type JeodeStatus =
  | "unknown"
  | "checking"
  | "installed"
  | "not_installed"
  | "installing"
  | "install_failed";

export type ProtonStatus = "idle" | "applying" | "applied" | "failed";

export interface SetupState {
  gameDirectory: string;
  jeodeStatus: JeodeStatus;
  onboardingComplete: boolean;
}
