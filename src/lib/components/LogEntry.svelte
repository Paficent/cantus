<script lang="ts">
    import type { LogEntry } from "$lib/types/logs";

    let { entry }: { entry: LogEntry } = $props();

    function color(level: string) {
        if (level === "info") return "text-blue-600 dark:text-blue-400";
        if (level === "warn") return "text-amber-600 dark:text-amber-400";
        if (level === "error") return "text-red-600 dark:text-red-400";
        return "text-muted-foreground";
    }
</script>

<div
    class="flex items-baseline gap-3 px-3 py-1 font-mono text-[12px] leading-relaxed hover:bg-accent/30 transition-colors"
>
    <span class="text-muted-foreground/60 shrink-0">{entry.timestamp}</span>
    <span class="shrink-0 w-11 {color(entry.level)}">{entry.level}</span>
    <span class="shrink-0 text-muted-foreground">[{entry.source}]</span>
    <span class="text-foreground min-w-0 break-all">{entry.message}</span>
</div>
