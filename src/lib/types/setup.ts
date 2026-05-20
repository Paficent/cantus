export type OnboardingStep = "directory" | "jeode";

export type JeodeStatus =
    | "unknown"
    | "checking"
    | "installed"
    | "not_installed"
    | "installing"
    | "install_failed";

export interface SetupState {
    gameDirectory: string;
    jeodeStatus: JeodeStatus;
    onboardingComplete: boolean;
}
