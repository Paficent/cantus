<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import { Switch } from "$lib/components/ui/switch";
    import * as Card from "$lib/components/ui/card";
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import { CODE_TO_JEODE } from "$lib/types/settings";
    import { Wrench } from "@lucide/svelte";

    type DangerKey = "allow_unsafe_functions" | "suppress_native_warnings";

    let pendingToggle = $state<DangerKey | null>(null);
    let listening = $state(false);

    function handleKey(e: KeyboardEvent) {
        if (!listening) return;
        e.preventDefault();
        e.stopPropagation();

        if (e.code === "Escape") {
            listening = false;
            return;
        }

        const k = CODE_TO_JEODE[e.code];
        if (k) settingsStore.updateJeode("toggle_key", k);
        listening = false;
    }

    function toggleDanger(key: DangerKey) {
        if (settingsStore.jeode[key]) {
            settingsStore.updateJeode(key, false);
        } else {
            pendingToggle = key;
        }
    }

    function confirmDanger() {
        if (pendingToggle) {
            settingsStore.updateJeode(pendingToggle, true);
            pendingToggle = null;
        }
    }

    const dialog = $derived.by(() => {
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
</script>

<svelte:window onkeydown={handleKey} />

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
                <p class="text-sm font-medium">UIs Visible on Startup</p>
                <p class="text-xs text-muted-foreground mt-0.5">
                    Whether Jeode mods with custom UIs are shown immediately
                </p>
            </div>
            <Switch
                checked={settingsStore.jeode.ui_visible_on_startup}
                onCheckedChange={(v) =>
                    settingsStore.updateJeode("ui_visible_on_startup", v)}
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
                onCheckedChange={(v) => settingsStore.updateJeode("debug", v)}
            />
        </div>

        <div class="h-px bg-border"></div>

        <Button
            variant="ghost"
            class="flex items-center justify-between gap-4 w-full h-auto py-0 px-0 hover:bg-transparent"
            onclick={() => toggleDanger("allow_unsafe_functions")}
        >
            <div class="text-left">
                <p class="text-sm font-medium">Allow unsafe Lua functions</p>
                <p class="text-xs text-muted-foreground mt-0.5 font-normal">
                    Disables the Lua sandbox for all mods
                </p>
            </div>
            <div class="pointer-events-none">
                <Switch checked={settingsStore.jeode.allow_unsafe_functions} />
            </div>
        </Button>

        <div class="h-px bg-border"></div>

        <Button
            variant="ghost"
            class="flex items-center justify-between gap-4 w-full h-auto py-0 px-0 hover:bg-transparent"
            onclick={() => toggleDanger("suppress_native_warnings")}
        >
            <div class="text-left">
                <p class="text-sm font-medium">Suppress native warnings</p>
                <p class="text-xs text-muted-foreground mt-0.5 font-normal">
                    Skip the confirmation when loading native mods
                </p>
            </div>
            <div class="pointer-events-none">
                <Switch
                    checked={settingsStore.jeode.suppress_native_warnings}
                />
            </div>
        </Button>

        <div class="h-px bg-border"></div>

        <div class="flex items-center justify-between gap-4">
            <div>
                <p class="text-sm font-medium">Toggle key</p>
                <p class="text-xs text-muted-foreground mt-0.5">
                    {#if listening}
                        Press a key or Escape to cancel
                    {:else}
                        Key to toggle the in-game overlay
                    {/if}
                </p>
            </div>
            <Button
                variant="outline"
                size="sm"
                class="font-mono min-w-[80px] {listening
                    ? 'border-primary ring-2 ring-primary/30 animate-pulse'
                    : ''}"
                onclick={() => (listening = true)}
            >
                {#if listening}
                    ...
                {:else}
                    {settingsStore.jeode.toggle_key}
                {/if}
            </Button>
        </div>
    </Card.Content>
</Card.Root>

<AlertDialog.Root
    open={pendingToggle !== null}
    onOpenChange={(open) => {
        if (!open) pendingToggle = null;
    }}
>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>{dialog.title}</AlertDialog.Title>
            <AlertDialog.Description
                >{dialog.description}</AlertDialog.Description
            >
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action
                class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                onclick={confirmDanger}
            >
                Enable
            </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
