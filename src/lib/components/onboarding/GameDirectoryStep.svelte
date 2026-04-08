<script lang="ts">
    import { Button, Input } from "$lib/components/ui";
    import { setupStore } from "$lib/stores/setup.svelte";
    import { FolderOpen, Loader2, ChevronRight } from "lucide-svelte";
</script>

<div class="flex flex-col gap-4">
    <div>
        <h2 class="text-base font-medium">Game path</h2>
        <p class="text-xs text-muted-foreground mt-0.5">
            Select your My Singing Monsters installation folder.
        </p>
    </div>

    <div class="flex-gap-2">
        <Input
            placeholder="C:\Program Files (x86)\Steam\steamapps\common\My Singing Monsters"
            value={setupStore.gameDirectory}
            oninput={(e: Event) => {
                setupStore.gameDirectory = (
                    e.currentTarget as HTMLInputElement
                ).value;
                setupStore.directoryValid = false;
                setupStore.error = "";
            }}
            class="flex-1 font-mono text-xs"
        />
    </div>

    {#if setupStore.error}
        <p class="text-xs text-destructive">{setupStore.error}</p>
    {/if}

    <div class="flex justify-between pt-2">
        <Button
            class="justify-start"
            variant="outline"
            onclick={() => setupStore.browseGameDirectory()}
        >
            <FolderOpen class="size-3.5" />
            Browse
        </Button>

        <div class="flex gap-2">
            {#if setupStore.gameDirectory && !setupStore.directoryValid && !setupStore.validating}
                <Button
                    variant="outline"
                    onclick={() => setupStore.validateDirectory()}
                >
                    Validate
                </Button>
            {/if}

            <Button
                disabled={!setupStore.directoryValid || setupStore.validating}
                onclick={() => setupStore.confirmDirectory()}
            >
                {#if setupStore.validating}
                    <Loader2 class="size-3.5 animate-spin" />
                    Validating...
                {:else}
                    Continue
                    <ChevronRight class="size-3.5" />
                {/if}
            </Button>
        </div>
    </div>
</div>
