<script lang="ts">
  import { cn } from "$lib/utils";

  interface Option {
    value: string;
    label: string;
  }

  interface Props {
    value?: string;
    options: Option[];
    class?: string;
    onchange?: (value: string) => void;
  }

  let { value, options, class: className, onchange }: Props = $props();
</script>

<select
  {value}
  class={cn(
    "flex h-9 rounded-md border border-input bg-background px-3 py-1 text-sm",
    "text-muted-foreground cursor-pointer",
    "focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
    className,
  )}
  onchange={(e) => onchange?.(e.currentTarget.value)}
>
  {#each options as opt}
    <option value={opt.value} selected={opt.value === value}>{opt.label}</option>
  {/each}
</select>
