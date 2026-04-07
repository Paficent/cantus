<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import { ModEntry } from "$lib/components/mods";
  import { modStore } from "$lib/stores/mods.svelte";
  import { FolderOpen, RefreshCw, Search, Puzzle } from "lucide-svelte";
  import type { Mod, TypeFilter, StatusFilter } from "$lib/types/mod";

  let removeTarget = $state<Mod | null>(null);

  function handleRemove(id: string) {
    removeTarget = modStore.getById(id) ?? null;
  }

  function confirmRemove() {
    if (removeTarget) {
      modStore.remove(removeTarget.id);
      removeTarget = null;
    }
  }
</script>

<div class="flex flex-col h-full">
  <div class="flex items-start justify-between mb-4">
    <div>
      <h2 class="text-base font-medium">Installed mods</h2>
      <p class="text-xs text-muted-foreground mt-0.5">
        {modStore.enabledCount} of {modStore.totalCount} enabled
      </p>
    </div>
    <div class="flex gap-2">
      <Button variant="outline" size="sm">
        <FolderOpen class="size-3.5" />
        Open mods folder
      </Button>
      <Button variant="outline" size="sm">
        <RefreshCw class="size-3.5" />
        Refresh
      </Button>
    </div>
  </div>

  <div class="flex gap-2 mb-3">
    <div class="relative flex-1">
      <Search class="absolute left-2.5 top-1/2 -translate-y-1/2 size-3.5 text-muted-foreground pointer-events-none" />
      <Input
        placeholder="Search mods..."
        value={modStore.search}
        oninput={(e: Event) => {
          modStore.search = (e.currentTarget as HTMLInputElement).value;
        }}
        class="pl-8"
      />
    </div>
    <select
      class="flex h-9 w-[130px] rounded-md border border-input bg-background px-3 py-1 text-sm text-muted-foreground cursor-pointer focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
      value={modStore.typeFilter}
      onchange={(e) => { modStore.typeFilter = (e.currentTarget as HTMLSelectElement).value as TypeFilter; }}
    >
      <option value="all">All types</option>
      <option value="native">Native</option>
      <option value="lua">Lua</option>
      <option value="asset">Asset</option>
    </select>
    <select
      class="flex h-9 w-[130px] rounded-md border border-input bg-background px-3 py-1 text-sm text-muted-foreground cursor-pointer focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
      value={modStore.statusFilter}
      onchange={(e) => { modStore.statusFilter = (e.currentTarget as HTMLSelectElement).value as StatusFilter; }}
    >
      <option value="all">All status</option>
      <option value="enabled">Enabled</option>
      <option value="disabled">Disabled</option>
    </select>
  </div>

  <div class="flex-1 rounded-lg border border-border overflow-y-auto">
    {#if modStore.filtered.length > 0}
      {#each modStore.filtered as mod (mod.id)}
        <ModEntry
          {mod}
          ontoggle={(id) => modStore.toggle(id)}
          onremove={handleRemove}
        />
      {/each}
    {:else}
      <div class="flex flex-col items-center justify-center py-16 text-muted-foreground">
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

<AlertDialog.Root open={removeTarget !== null} onOpenChange={(open) => { if (!open) removeTarget = null; }}>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>Remove {removeTarget?.name}?</AlertDialog.Title>
      <AlertDialog.Description>
        This will delete the mod folder from your mods directory and cannot be undone.
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
