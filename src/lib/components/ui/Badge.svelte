<script lang="ts">
  import { cn } from "$lib/utils";
  import type { Snippet } from "svelte";

  interface Props {
    variant?: "default" | "secondary" | "destructive";
    class?: string;
    children: Snippet;
  }

  let { variant = "default", class: className, children }: Props = $props();

  const variants = {
    default: "bg-primary/10 text-primary",
    secondary: "bg-secondary text-muted-foreground",
    destructive: "bg-destructive/10 text-destructive",
  };
</script>

<span class={cn(
  "inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[11px] font-mono",
  variants[variant],
  className,
)}>
  {@render children()}
</span>
