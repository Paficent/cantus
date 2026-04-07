<script lang="ts">
  import Button from "./Button.svelte";

  interface Props {
    open?: boolean;
    title: string;
    description: string;
    onconfirm?: () => void;
    oncancel?: () => void;
  }

  let { open = false, title, description, onconfirm, oncancel }: Props = $props();
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    <div class="fixed inset-0 bg-black/40" onclick={oncancel}></div>
    <div class="relative z-50 w-full max-w-md rounded-lg border border-border bg-background p-6 shadow-lg">
      <h3 class="text-base font-medium">{title}</h3>
      <p class="mt-2 text-sm text-muted-foreground leading-relaxed">{description}</p>
      <div class="mt-5 flex justify-end gap-2">
        <Button variant="outline" size="sm" onclick={oncancel}>Cancel</Button>
        <Button variant="destructive" size="sm" onclick={onconfirm}>Remove</Button>
      </div>
    </div>
  </div>
{/if}
