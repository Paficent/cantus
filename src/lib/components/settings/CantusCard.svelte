<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import { Switch } from "$lib/components/ui/switch";
    import * as Card from "$lib/components/ui/card";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import { setupStore } from "$lib/stores/setup.svelte";
    import { updaterStore } from "$lib/stores/updater.svelte";
    import {
        FolderOpen,
        Settings,
        Scan,
        Sun,
        Moon,
        Download,
        Loader2,
        RefreshCw,
    } from "@lucide/svelte";
    import { invoke } from "@tauri-apps/api/core";

    async function autodetect() {
        const detected = await invoke<string | null>("detect_game_directory");
        if (!detected) return;
        const valid = await invoke<boolean>("validate_game_directory", {
            path: detected,
        });
        if (!valid) return;
        setupStore.gameDirectory = detected;
        await settingsStore.updateCantus("game_directory", detected);
    }

    async function browseDir() {
        const selected = await invoke<string | null>("browse_game_directory");
        if (!selected) return;
        const valid = await invoke<boolean>("validate_game_directory", {
            path: selected,
        });
        if (!valid) return;
        setupStore.gameDirectory = selected;
        await settingsStore.updateCantus("game_directory", selected);
    }

    const isDark = $derived(settingsStore.cantus.theme === "dark");
</script>

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
                <p class="text-sm font-medium">Version</p>
                <p class="text-xs text-muted-foreground mt-0.5">
                    {#if updaterStore.installing}
                        Downloading v{updaterStore.pending?.version}... {updaterStore.progressPct}%
                    {:else if updaterStore.pending}
                        v{updaterStore.currentVersion || "?"} — v{updaterStore
                            .pending.version} available
                    {:else if updaterStore.checking}
                        v{updaterStore.currentVersion || "?"} — checking...
                    {:else}
                        v{updaterStore.currentVersion || "?"} — up to date
                    {/if}
                </p>
            </div>
            <div class="flex items-center gap-1.5">
                {#if updaterStore.installing}
                    <Button variant="outline" size="sm" disabled>
                        <Loader2 class="size-3.5 animate-spin" />
                        Installing
                    </Button>
                {:else if updaterStore.pending}
                    <Button size="sm" onclick={() => updaterStore.install()}>
                        <Download class="size-3.5" />
                        Install
                    </Button>
                {:else}
                    <Button
                        variant="outline"
                        size="sm"
                        disabled={updaterStore.checking}
                        onclick={() => updaterStore.checkManual()}
                    >
                        {#if updaterStore.checking}
                            <Loader2 class="size-3.5 animate-spin" />
                            Checking
                        {:else}
                            <RefreshCw class="size-3.5" />
                            Check
                        {/if}
                    </Button>
                {/if}
            </div>
        </div>

        <div class="h-px bg-border"></div>

        <div class="flex items-center justify-between gap-4">
            <div class="min-w-0 flex-1">
                <p class="text-sm font-medium">Game directory</p>
                <p
                    class="text-xs text-muted-foreground font-mono truncate mt-0.5"
                >
                    {settingsStore.cantus.game_directory || "Not set"}
                </p>
            </div>
            <div class="flex items-center gap-1.5">
                <Button variant="outline" size="sm" onclick={autodetect}>
                    <Scan class="size-3.5" />
                    Autodetect
                </Button>
                <Button variant="outline" size="sm" onclick={browseDir}>
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
                <p class="text-sm font-medium">Show NSFW content</p>
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
