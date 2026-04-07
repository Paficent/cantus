<script lang="ts">
  interface Props {
    checked?: boolean;
    onchange?: (checked: boolean) => void;
  }

  let { checked = false, onchange }: Props = $props();
</script>

<button
  role="switch"
  aria-checked={checked}
  class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors
    {checked ? 'bg-primary' : 'bg-input'}"
  onclick={() => onchange?.(!checked)}
>
  <span
    class="pointer-events-none block h-4 w-4 rounded-full bg-background shadow-sm ring-0 transition-transform
      {checked ? 'translate-x-4' : 'translate-x-0'}"
  ></span>
</button>
