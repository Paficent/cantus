<script lang="ts">
  import { cn } from "$lib/utils";

  interface Props {
    value?: string;
    placeholder?: string;
    class?: string;
    oninput?: (value: string) => void;
  }

  let { value = "", placeholder, class: className, oninput }: Props = $props();
</script>

<input
  type="text"
  {value}
  {placeholder}
  class={cn(
    "flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm",
    "placeholder:text-muted-foreground",
    "focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
    className,
  )}
  oninput={(e) => oninput?.(e.currentTarget.value)}
/>
