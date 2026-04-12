<script lang="ts">
    import { onMount } from "svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import { Switch } from "$lib/components/ui/switch";
    import * as Card from "$lib/components/ui/card";
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import { setupStore } from "$lib/stores/setup.svelte";
    import { CODE_TO_JEODE } from "$lib/types/settings";
    import {
        Loader2,
        FolderOpen,
        Settings,
        Wrench,
        Scan,
        Sun,
        Moon,
    } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";

    type DangerousKey = "allow_unsafe_functions" | "suppress_native_warnings";

    let pendingToggle = $state<DangerousKey | null>(null);
    let listeningForKey = $state(false);

    onMount(() => {
        settingsStore.load();
        settingsStore.startWatching();
    });

    function startKeyCapture() {
        listeningForKey = true;
    }

    function handleKeyCapture(e: KeyboardEvent) {
        if (!listeningForKey) return;

        e.preventDefault();
        e.stopPropagation();

        if (e.code === "Escape") {
            listeningForKey = false;
            return;
        }

        const jeodeKey = CODE_TO_JEODE[e.code];
        if (jeodeKey) {
            settingsStore.updateJeode("toggle_key", jeodeKey);
        }

        listeningForKey = false;
    }

    async function handleAutodetect() {
        try {
            const detected = await invoke<string | null>(
                "detect_game_directory",
            );
            if (!detected) return;

            const valid = await invoke<boolean>("validate_game_directory", {
                path: detected,
            });
            if (!valid) return;

            setupStore.gameDirectory = detected;
            await settingsStore.updateCantus("game_directory", detected);
        } catch (e) {
            console.error("Failed to autodetect game directory:", e);
        }
    }

    async function handleBrowseDirectory() {
        try {
            const selected = await invoke<string | null>(
                "browse_game_directory",
            );
            if (!selected) return;

            const valid = await invoke<boolean>("validate_game_directory", {
                path: selected,
            });
            if (!valid) return;

            setupStore.gameDirectory = selected;
            await settingsStore.updateCantus("game_directory", selected);
        } catch (e) {
            console.error("Failed to browse game directory:", e);
        }
    }

    function handleDangerousToggle(key: DangerousKey) {
        if (settingsStore.jeode[key]) {
            settingsStore.updateJeode(key, false);
        } else {
            pendingToggle = key;
        }
    }

    function confirmDangerousToggle() {
        if (pendingToggle) {
            settingsStore.updateJeode(pendingToggle, true);
            pendingToggle = null;
        }
    }

    const dangerousDialogMeta = $derived.by(() => {
        if (pendingToggle === "allow_unsafe_functions") {
            return {
                title: "Allow unsafe Lua functions?",
                description:
                    "This disables the Lua sandbox, giving mods unrestricted access to system APIs. Only enable this if you trust all installed mods.",
            };
        }
        return {
            title: "Suppress native mod warnings?",
            description:
                "This removes the confirmation dialog shown when loading native mods. Native mods run arbitrary code and can modify your system.",
        };
    });

    const isDark = $derived(settingsStore.cantus.theme === "dark");
</script>

<svelte:window onkeydown={handleKeyCapture} />

<div class="flex flex-col h-full">
    <div class="flex items-start justify-between mb-4">
        <div>
            <h2 class="text-base font-medium">Settings</h2>
        </div>
    </div>

    {#if settingsStore.loading}
        <div
            class="flex flex-col items-center justify-center flex-1 text-muted-foreground"
        >
            <Loader2 class="size-10 mb-3 opacity-30 animate-spin" />
            <p class="text-sm">Loading settings...</p>
        </div>
    {:else}
        <div class="flex-1 overflow-y-auto pr-1 space-y-4">
            <Card.Root>
                <Card.Header>
                    <div class="flex items-center gap-2">
                        <Settings class="size-4 text-muted-foreground" />
                        <Card.Title>Cantus</Card.Title>
                    </div>
                    <Card.Description>Manager settings</Card.Description>
                </Card.Header>
                <Card.Content class="space-y-4">
                    <div class="flex items-center justify-between gap-4">
                        <div class="min-w-0 flex-1">
                            <p class="text-sm font-medium">Game directory</p>
                            <p
                                class="text-xs text-muted-foreground font-mono truncate mt-0.5"
                            >
                                {settingsStore.cantus.game_directory ||
                                    "Not set"}
                            </p>
                        </div>
                        <div class="flex items-center gap-1.5">
                            <Button
                                variant="outline"
                                size="sm"
                                onclick={handleAutodetect}
                            >
                                <Scan class="size-3.5" />
                                Autodetect
                            </Button>
                            <Button
                                variant="outline"
                                size="sm"
                                onclick={handleBrowseDirectory}
                            >
                                <FolderOpen class="size-3.5" />
                                Browse
                            </Button>
                        </div>
                    </div>

                    <div class="h-px bg-border"></div>

                    <div class="flex items-center justify-between gap-4">
                        <div>
                            <p class="text-sm font-medium">Theme</p>
                            <p class="text-xs text-muted-foreground mt-0.5">
                                Switch between dark and light mode
                            </p>
                        </div>
                        <Button
                            variant="outline"
                            size="sm"
                            onclick={() =>
                                settingsStore.updateCantus(
                                    "theme",
                                    isDark ? "light" : "dark",
                                )}
                        >
                            {#if isDark}
                                <Moon class="size-3.5" />
                                Dark
                            {:else}
                                <Sun class="size-3.5" />
                                Light
                            {/if}
                        </Button>
                    </div>

                    <div class="h-px bg-border"></div>

                    <div class="flex items-center justify-between gap-4">
                        <div>
                            <p class="text-sm font-medium">
                                Show NSFW content
                            </p>
                            <p class="text-xs text-muted-foreground mt-0.5">
                                Display age-restricted mods in the browse tab
                            </p>
                        </div>
                        <Switch
                            checked={settingsStore.cantus.show_nsfw}
                            onCheckedChange={(v) =>
                                settingsStore.updateCantus("show_nsfw", v)}
                        />
                    </div>
                </Card.Content>
            </Card.Root>

            <Card.Root>
                <Card.Header>
                    <div class="flex items-center gap-2">
                        <Wrench class="size-4 text-muted-foreground" />
                        <Card.Title>Jeode</Card.Title>
                    </div>
                    <Card.Description>Loader settings</Card.Description>
                </Card.Header>
                <Card.Content class="space-y-4">
                    <div class="flex items-center justify-between gap-4">
                        <div>
                            <p class="text-sm font-medium">
                                Overlays enabled
                            </p>
                            <p class="text-xs text-muted-foreground mt-0.5">
                                Show the in-game overlay
                            </p>
                        </div>
                        <Switch
                            checked={settingsStore.jeode.overlays_enabled}
                            onCheckedChange={(v) =>
                                settingsStore.updateJeode(
                                    "overlays_enabled",
                                    v,
                                )}
                        />
                    </div>

                    <div class="h-px bg-border"></div>

                    <div class="flex items-center justify-between gap-4">
                        <div>
                            <p class="text-sm font-medium">Debug mode</p>
                            <p class="text-xs text-muted-foreground mt-0.5">
                                Enable verbose logging output
                            </p>
                        </div>
                        <Switch
                            checked={settingsStore.jeode.debug}
                            onCheckedChange={(v) =>
                                settingsStore.updateJeode("debug", v)}
                        />
                    </div>

                    <div class="h-px bg-border"></div>

                    <Button
                        variant="ghost"
                        class="flex items-center justify-between gap-4 w-full h-auto py-0 px-0 hover:bg-transparent"
                        onclick={() =>
                            handleDangerousToggle("allow_unsafe_functions")}
                    >
                        <div class="text-left">
                            <p class="text-sm font-medium">
                                Allow unsafe Lua functions
                            </p>
                            <p class="text-xs text-muted-foreground mt-0.5 font-normal">
                                Disables the Lua sandbox for all mods
                            </p>
                        </div>
                        <div class="pointer-events-none">
                            <Switch
                                checked={settingsStore.jeode
                                    .allow_unsafe_functions}
                            />
                        </div>
                    </Button>

                    <div class="h-px bg-border"></div>

                    <Button
                        variant="ghost"
                        class="flex items-center justify-between gap-4 w-full h-auto py-0 px-0 hover:bg-transparent"
                        onclick={() =>
                            handleDangerousToggle(
                                "suppress_native_warnings",
                            )}
                    >
                        <div class="text-left">
                            <p class="text-sm font-medium">
                                Suppress native warnings
                            </p>
                            <p class="text-xs text-muted-foreground mt-0.5 font-normal">
                                Skip the confirmation when loading native
                                mods
                            </p>
                        </div>
                        <div class="pointer-events-none">
                            <Switch
                                checked={settingsStore.jeode
                                    .suppress_native_warnings}
                            />
                        </div>
                    </Button>

                    <div class="h-px bg-border"></div>

                    <!-- TODO: Use kbd component instead -->
                    <div class="flex items-center justify-between gap-4">
                        <div>
                            <p class="text-sm font-medium">Toggle key</p>
                            <p class="text-xs text-muted-foreground mt-0.5">
                                {#if listeningForKey}
                                    Press a key or Escape to cancel
                                {:else}
                                    Key to toggle the in-game overlay
                                {/if}
                            </p>
                        </div>
                        <Button
                            variant="outline"
                            size="sm"
                            class="font-mono min-w-[80px] {listeningForKey
                                ? 'border-primary ring-2 ring-primary/30 animate-pulse'
                                : ''}"
                            onclick={startKeyCapture}
                        >
                            {#if listeningForKey}
                                ...
                            {:else}
                                {settingsStore.jeode.toggle_key}
                            {/if}
                        </Button>
                    </div>
                </Card.Content>
            </Card.Root>
        </div>
    {/if}
</div>

<AlertDialog.Root
    open={pendingToggle !== null}
    onOpenChange={(open) => {
        if (!open) pendingToggle = null;
    }}
>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>{dangerousDialogMeta.title}</AlertDialog.Title>
            <AlertDialog.Description>
                {dangerousDialogMeta.description}
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action
                class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                onclick={confirmDangerousToggle}
            >
                Enable
            </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
