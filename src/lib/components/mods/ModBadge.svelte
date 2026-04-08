<script lang="ts">
    import { Badge } from "$lib/components/ui/badge";
    import { Cpu, FileCode, Image } from "lucide-svelte";
    import type { ModType } from "$lib/types/mod";

    interface Props {
        type: ModType;
    }

    let { type }: Props = $props();

    const config = {
        native: { label: "Native", variant: "destructive" as const, icon: Cpu },
        lua: { label: "Lua", variant: "default" as const, icon: FileCode },
        asset: { label: "Asset", variant: "secondary" as const, icon: Image },
    };

    const current = $derived(config[type]);
</script>

<Badge variant={current.variant} class="gap-1 text-[11px] font-mono">
    <current.icon class="size-3" />
    {current.label}
</Badge>
