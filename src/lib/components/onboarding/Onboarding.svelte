<script lang="ts">
    import { setupStore } from "$lib/stores/setup.svelte";
    import GameStep from "./GameStep.svelte";
    import JeodeStep from "./JeodeStep.svelte";
</script>

<div class="flex h-screen items-center justify-center select-none">
    <div class="w-full max-w-md">
        <div class="mb-6 flex items-center gap-2.5">
            <div class="size-8 rounded-md bg-primary flex items-center justify-center">
                <span class="text-primary-foreground text-sm font-bold">C</span>
            </div>
            <div>
                <p class="text-sm font-medium leading-none">Cantus</p>
                <p class="text-[10px] text-muted-foreground mt-0.5">Setup Wizard</p>
            </div>
        </div>

        <div class="rounded-xl border border-border bg-card p-6">
            <div class="flex gap-1.5 mb-6">
                <div
                    class="h-1 flex-1 rounded-full {setupStore.step === 'directory'
                        ? 'bg-primary'
                        : 'bg-primary/30'}"
                ></div>
                <div
                    class="h-1 flex-1 rounded-full {setupStore.step === 'jeode'
                        ? 'bg-primary'
                        : 'bg-muted'}"
                ></div>
            </div>

            {#if setupStore.step === "directory"}
                <GameStep />
            {:else if setupStore.step === "jeode"}
                <JeodeStep />
            {/if}
        </div>
    </div>
</div>
