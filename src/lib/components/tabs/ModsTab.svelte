<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import Input from "$lib/components/ui/input/input.svelte";
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import * as Select from "$lib/components/ui/select";
    import { ModEntry } from "$lib/components/mods";
    import { modStore } from "$lib/stores/mods.svelte";
    import {
        RefreshCw,
        Search,
        Puzzle,
        Loader2,
        Download,
    } from "lucide-svelte";
    import { toast } from "svelte-sonner";
    import type {
        Mod,
        InstallResult,
        TypeFilter,
        StatusFilter,
    } from "$lib/types/mod";

    function showInstallToast(result: InstallResult) {
        if (result.installed.length === result.total) {
            const names = result.installed.join(", ");
            toast.success(
                result.total === 1
                    ? `Installed ${names}`
                    : `Installed ${result.total} mods`,
                result.total > 1 ? { description: names } : undefined,
            );
        } else if (result.installed.length > 0) {
            toast.warning(
                `Installed ${result.installed.length} of ${result.total} mods`,
                {
                    description: result.error ?? undefined,
                },
            );
        } else {
            toast.error("Installation failed", {
                description: result.error ?? undefined,
            });
        }
    }

    const typeLabels: Record<TypeFilter, string> = {
        all: "All types",
        native: "Native",
        lua: "Lua",
        asset: "Asset",
    };
    const statusLabels: Record<StatusFilter, string> = {
        all: "All status",
        enabled: "Enabled",
        disabled: "Disabled",
    };

    let removeTarget = $state<Mod | null>(null);

    $effect(() => {
        modStore.load();
        modStore.startWatching();
    });

    function handleRemove(id: string) {
        removeTarget = modStore.getById(id) ?? null;
    }

    async function confirmRemove() {
        if (removeTarget) {
            await modStore.remove(removeTarget.id);
            removeTarget = null;
        }
    }
</script>

<div class="flex flex-col h-full">
    <div class="flex items-start justify-between mb-4">
        <div>
            <h2 class="text-base font-medium">Installed Mods</h2>
            <p class="text-xs text-muted-foreground mt-0.5">
                {modStore.enabledCount} of {modStore.totalCount} enabled
            </p>
        </div>
        <div class="flex items-center gap-2">
            <Button variant="outline" size="sm" onclick={() => modStore.load()}>
                <RefreshCw class="size-3.5" />
                Refresh
            </Button>
            <Button
                size="sm"
                disabled={modStore.installing}
                onclick={async () => {
                    try {
                        const result = await modStore.install();
                        if (result) showInstallToast(result);
                    } catch (e) {
                        toast.error("Installation failed", {
                            description: String(e),
                        });
                    }
                }}
            >
                {#if modStore.installing}
                    <Loader2 class="size-3.5 animate-spin" />
                    Installing...
                {:else}
                    <Download class="size-3.5" />
                    Install mod
                {/if}
            </Button>
        </div>
    </div>

    <div class="flex gap-2 mb-3">
        <div class="relative flex-1">
            <Search
                class="absolute left-2.5 top-1/2 -translate-y-1/2 size-3.5 text-muted-foreground pointer-events-none"
            />
            <Input
                placeholder="Search mods..."
                value={modStore.search}
                oninput={(e: Event) => {
                    modStore.search = (
                        e.currentTarget as HTMLInputElement
                    ).value;
                }}
                class="pl-8"
            />
        </div>
        <Select.Root
            type="single"
            value={modStore.typeFilter}
            onValueChange={(v) => {
                if (v) modStore.typeFilter = v as TypeFilter;
            }}
        >
            <Select.Trigger class="w-[130px]">
                {typeLabels[modStore.typeFilter]}
            </Select.Trigger>
            <Select.Content>
                <Select.Item value="all">All types</Select.Item>
                <Select.Item value="native">Native</Select.Item>
                <Select.Item value="lua">Lua</Select.Item>
                <Select.Item value="asset">Asset</Select.Item>
            </Select.Content>
        </Select.Root>
        <Select.Root
            type="single"
            value={modStore.statusFilter}
            onValueChange={(v) => {
                if (v) modStore.statusFilter = v as StatusFilter;
            }}
        >
            <Select.Trigger class="w-[130px]">
                {statusLabels[modStore.statusFilter]}
            </Select.Trigger>
            <Select.Content>
                <Select.Item value="all">All status</Select.Item>
                <Select.Item value="enabled">Enabled</Select.Item>
                <Select.Item value="disabled">Disabled</Select.Item>
            </Select.Content>
        </Select.Root>
    </div>

    <div class="flex-1 rounded-lg border border-border overflow-y-auto">
        {#if modStore.loading}
            <div
                class="flex flex-col items-center justify-center py-16 text-muted-foreground"
            >
                <Loader2 class="size-10 mb-3 opacity-30 animate-spin" />
                <p class="text-sm">Loading mods...</p>
            </div>
        {:else if modStore.filtered.length > 0}
            {#each modStore.filtered as mod (mod.id)}
                <ModEntry
                    {mod}
                    ontoggle={(id) => modStore.toggle(id)}
                    onremove={handleRemove}
                    onopenfolder={(id) => modStore.openModFolder(id)}
                />
            {/each}
        {:else}
            <div
                class="flex flex-col items-center justify-center py-16 text-muted-foreground"
            >
                <Puzzle class="size-10 mb-3 opacity-30" />
                <p class="text-sm">No mods found</p>
                <p class="text-xs mt-1">
                    {#if modStore.search}
                        Try a different search
                    {:else}
                        Install mods manually or in the Browse page
                    {/if}
                </p>
            </div>
        {/if}
    </div>
</div>

<AlertDialog.Root
    open={removeTarget !== null}
    onOpenChange={(open) => {
        if (!open) removeTarget = null;
    }}
>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Remove {removeTarget?.name}?</AlertDialog.Title>
            <AlertDialog.Description>
                This will delete the mod folder from your mods directory and
                cannot be undone.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action
                class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                onclick={confirmRemove}
            >
                Remove
            </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
