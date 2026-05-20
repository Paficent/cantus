<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import { setupStore } from "$lib/stores/setup.svelte";
    import {
        CheckCircle2,
        Loader2,
        XCircle,
        ChevronLeft,
        Download,
        RefreshCw,
    } from "@lucide/svelte";

    const status = $derived(setupStore.jeodeStatus);
</script>

<div class="flex flex-col gap-4">
    <div>
        <h2 class="text-base font-medium">Jeode</h2>
        <p class="text-xs text-muted-foreground mt-0.5">
            Cantus requires you to have the Jeode mod loader installed.
        </p>
    </div>

    <div class="rounded-lg border border-border p-4">
        {#if status === "checking"}
            <div class="flex items-center gap-3 text-muted-foreground">
                <Loader2 class="size-4 animate-spin" />
                <p class="text-sm">Checking for Jeode installation...</p>
            </div>
        {:else if status === "installed"}
            <div class="flex items-center gap-3 text-emerald-500">
                <CheckCircle2 class="size-4" />
                <p class="text-sm">Jeode is installed.</p>
            </div>
        {:else if status === "installing"}
            <div class="flex items-center gap-3 text-muted-foreground">
                <Loader2 class="size-4 animate-spin" />
                <p class="text-sm">Installing Jeode...</p>
            </div>
        {:else if status === "install_failed"}
            <div class="flex items-center gap-3 text-destructive">
                <XCircle class="size-4" />
                <p class="text-sm">Jeode installation failed.</p>
            </div>
        {:else if status === "not_installed"}
            <div class="flex items-center gap-3 text-muted-foreground">
                <XCircle class="size-4" />
                <p class="text-sm">Jeode was not detected.</p>
            </div>
        {/if}
    </div>

    {#if setupStore.error}
        <p class="text-xs text-destructive">{setupStore.error}</p>
    {/if}

    <div class="flex justify-between pt-2">
        <Button
            variant="outline"
            disabled={status === "installing" || status === "checking"}
            onclick={() => setupStore.goBack()}
        >
            <ChevronLeft class="size-3.5" />
            Back
        </Button>

        <div class="flex gap-2">
            {#if status === "not_installed"}
                <Button onclick={() => setupStore.installJeode()}>
                    <Download class="size-3.5" />
                    Install Jeode
                </Button>
            {:else if status === "install_failed"}
                <Button
                    variant="outline"
                    onclick={() => setupStore.checkJeode()}
                >
                    <RefreshCw class="size-3.5" />
                    Retry
                </Button>
                <Button onclick={() => setupStore.installJeode()}>
                    <Download class="size-3.5" />
                    Try again
                </Button>
            {:else if status === "installed"}
                <Button onclick={() => setupStore.finishOnboarding()}>
                    Finish setup
                    <CheckCircle2 class="size-3.5" />
                </Button>
            {/if}
        </div>
    </div>
</div>
