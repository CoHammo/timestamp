<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { init } from "./state.svelte";
    import "../app.css";

    let { children } = $props();

    onMount(async () => {
        await init();
    });
</script>

{@render children()}
