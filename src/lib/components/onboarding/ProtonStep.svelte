<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import { setupStore } from "$lib/stores/setup.svelte";
    import {
        CheckCircle2,
        Loader2,
        XCircle,
        ChevronLeft,
        Terminal,
        RefreshCw,
    } from "@lucide/svelte";

    const status = $derived(setupStore.protonStatus);
</script>

<div class="flex flex-col gap-4">
    <div>
        <h2 class="text-base font-medium">Wine / Proton</h2>
        <p class="text-xs text-muted-foreground mt-0.5">
            Looks like you're running through Wine or Proton. Jeode won't load
            until <code class="text-foreground">winhttp</code> is added to your prefix's
            DLL overrides.
        </p>
    </div>

    <div class="rounded-lg border border-border p-4">
        {#if status === "applying"}
            <div class="flex items-center gap-3 text-muted-foreground">
                <Loader2 class="size-4 animate-spin" />
                <p class="text-sm">Adding the override...</p>
            </div>
        {:else if status === "applied"}
            <div class="flex items-center gap-3 text-emerald-500">
                <CheckCircle2 class="size-4" />
                <p class="text-sm">Override added.</p>
            </div>
        {:else if status === "failed"}
            <div class="flex items-center gap-3 text-destructive">
                <XCircle class="size-4" />
                <p class="text-sm">Couldn't add the override automatically.</p>
            </div>
        {:else}
            <p class="text-sm">
                Add the <code class="text-foreground">winhttp</code> DLL override
                (native, builtin) to your Wine prefix now?
            </p>
        {/if}
    </div>

    {#if setupStore.error}
        <p class="text-xs text-destructive">{setupStore.error}</p>
    {/if}

    <div class="flex justify-between pt-2">
        <Button
            variant="outline"
            disabled={status === "applying"}
            onclick={() => setupStore.goBack()}
        >
            <ChevronLeft class="size-3.5" />
            Back
        </Button>

        <div class="flex gap-2">
            {#if status === "failed"}
                <Button
                    variant="outline"
                    onclick={() => setupStore.skipProton()}
                >
                    Skip
                </Button>
                <Button onclick={() => setupStore.applyProtonOverride()}>
                    <RefreshCw class="size-3.5" />
                    Try again
                </Button>
            {:else if status !== "applying"}
                <Button
                    variant="outline"
                    onclick={() => setupStore.skipProton()}
                >
                    No, skip
                </Button>
                <Button onclick={() => setupStore.applyProtonOverride()}>
                    <Terminal class="size-3.5" />
                    Yes, apply
                </Button>
            {/if}
        </div>
    </div>
</div>
