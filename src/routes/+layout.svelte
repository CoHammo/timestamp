<script lang="ts">
    import { PunchEditor } from "$lib/components";
    import type { Punch } from "$lib/types";
    import { onMount, setContext } from "svelte";
    import { init } from "$lib/commands";
    import "../app.css";

    let { children } = $props();
    let punchEditor: PunchEditor | undefined = $state();

    onMount(async () => {
        await init();
    });

    setContext("editPunch", (p: Punch, listIndex: number) =>
        punchEditor?.open(p, listIndex),
    );
</script>

{@render children()}

<PunchEditor bind:this={punchEditor} />
