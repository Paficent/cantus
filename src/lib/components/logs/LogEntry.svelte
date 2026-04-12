<script lang="ts">
    import type { LogEntry } from "$lib/types/logs";

    interface Props {
        entry: LogEntry;
    }

    let { entry }: Props = $props();

    const levelColors = {
        info: "text-blue-600 dark:text-blue-400",
        debug: "text-muted-foreground",
        warn: "text-amber-600 dark:text-amber-400",
        error: "text-red-600 dark:text-red-400",
    };
</script>

<div
    class="flex items-baseline gap-3 px-3 py-1 font-mono text-[12px] leading-relaxed hover:bg-accent/30 transition-colors"
>
    <span class="text-muted-foreground/60 shrink-0">{entry.timestamp}</span>
    <span class="shrink-0 w-11 {levelColors[entry.level]}">{entry.level}</span>
    <span class="shrink-0 text-muted-foreground">[{entry.source}]</span>
    <span class="text-foreground min-w-0 break-all">{entry.message}</span>
</div>
