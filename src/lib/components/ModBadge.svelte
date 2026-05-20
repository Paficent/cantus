<script lang="ts">
    import { Badge } from "$lib/components/ui/badge";
    import { Cpu, FileCode, Image } from "@lucide/svelte";
    import type { ModType } from "$lib/types/mod";

    let { type }: { type: ModType } = $props();

    const variant = $derived(
        type === "native"
            ? "destructive"
            : type === "lua"
              ? "default"
              : "secondary",
    );
    const Icon = $derived(
        type === "native" ? Cpu : type === "lua" ? FileCode : Image,
    );
    const label = $derived(type[0].toUpperCase() + type.slice(1));
</script>

<Badge {variant} class="gap-1 text-[11px] font-mono">
    <Icon class="size-3" />
    {label}
</Badge>
