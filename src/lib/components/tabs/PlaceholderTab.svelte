<script lang="ts">
    import { Code } from "@lucide/svelte";

    let { title, description }: { title: string; description: string } =
        $props();
</script>

<div
    class="flex flex-col items-center justify-center h-full text-muted-foreground"
>
    <Code class="size-10 mb-3 opacity-30" />
    <p class="text-sm font-medium">{title}</p>
    <p class="text-xs mt-1">{description || "This is not yet implemented"}</p>
</div>
