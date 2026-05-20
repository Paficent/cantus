<script lang="ts">
    import { Switch } from "$lib/components/ui/switch";
    import Button from "$lib/components/ui/button/button.svelte";
    import ModBadge from "$lib/components/ModBadge.svelte";
    import { Trash2, FolderOpen } from "@lucide/svelte";
    import type { Mod } from "$lib/types/mod";

    let {
        mod,
        ontoggle,
        onremove,
        onopenfolder,
    }: {
        mod: Mod;
        ontoggle: (id: string) => void;
        onremove: (id: string) => void;
        onopenfolder: (id: string) => void;
    } = $props();
</script>

<div
    class="flex items-center gap-3.5 px-4 py-3 border-b border-border last:border-b-0 transition-opacity"
    class:opacity-45={!mod.enabled}
>
    <Switch checked={mod.enabled} onCheckedChange={() => ontoggle(mod.id)} />

    <div class="flex-1 min-w-0">
        <div class="flex items-center gap-2">
            <span class="text-sm font-medium truncate">{mod.name}</span>
            <ModBadge type={mod.type} />
        </div>
        <div class="flex items-center gap-3 mt-0.5">
            <span class="text-[11px] text-muted-foreground font-mono"
                >{mod.id}</span
            >
            <span class="text-[11px] text-muted-foreground">v{mod.version}</span
            >
            <span class="text-[11px] text-muted-foreground"
                >by {mod.author}</span
            >
        </div>
    </div>

    <div class="flex shrink-0">
        <Button
            variant="ghost"
            size="icon"
            class="text-muted-foreground hover:text-foreground"
            onclick={() => onopenfolder(mod.id)}
        >
            <FolderOpen class="size-4" />
        </Button>
        <Button
            variant="ghost"
            size="icon"
            class="text-muted-foreground hover:text-destructive hover:bg-destructive/10"
            onclick={() => onremove(mod.id)}
        >
            <Trash2 class="size-4" />
        </Button>
    </div>
</div>
