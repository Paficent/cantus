<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import { updaterStore } from "$lib/stores/updater.svelte";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { Download } from "@lucide/svelte";

    function openReleaseNotes(e: MouseEvent) {
        e.preventDefault();
        if (updaterStore.releaseUrl) openUrl(updaterStore.releaseUrl);
    }

    function install() {
        updaterStore.dialogOpen = false;
        updaterStore.install();
    }
</script>

<AlertDialog.Root bind:open={updaterStore.dialogOpen}>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Update available</AlertDialog.Title>
            <AlertDialog.Description>
                Cantus v{updaterStore.pending?.version} is available
                {#if updaterStore.currentVersion}
                    (you're on v{updaterStore.currentVersion}){/if}. See the
                <a
                    href={updaterStore.releaseUrl ?? "#"}
                    onclick={openReleaseNotes}>release notes</a
                > or install now to update.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Not now</AlertDialog.Cancel>
            <AlertDialog.Action onclick={install}>
                <Download class="size-3.5" />
                Install update
            </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
