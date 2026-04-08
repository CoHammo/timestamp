<script lang="ts">
    import { PunchCard } from "$lib";
    import { onMount } from "svelte";
    import { jobs, punches, state } from "./state.svelte";
    import Menu from "@lucide/svelte/icons/menu";
    import Settings from "@lucide/svelte/icons/settings";
    import { invoke } from "@tauri-apps/api/core";

    // onMount(async () => {
    //     jobs.list = await invoke("get_jobs");
    //     punches.list = await invoke("get_punches", { jobId: 1 });
    //     console.log(JSON.stringify(punches.list, null, 2));
    // });
</script>

<main class="h-screen">
    <!-- App Bar -->
    <div
        class="w-full flex bg-blue-600 justify-between text-white px-2 py-3 min-h-18 items-center"
    >
        <div class="flex items-center gap-0.5">
            <button
                class="btn btn-circle border-none bg-none hover:bg-white/20 h-10 w-10"
            >
                <Menu />
            </button>
            <button
                class="btn border-none bg-none hover:bg-white/20 px-2 rounded-md"
            >
                <h1 class="text-[1.3rem] font-bold">{state.state?.job.name}</h1>
            </button>
        </div>
        <div>
            <button
                class="btn btn-circle border-none bg-none hover:bg-white/20 h-10 w-10"
            >
                <Settings />
            </button>
        </div>
    </div>

    <!-- List of Punches for the Current Job -->
    <div class="flex flex-col gap-2 p-2">
        {#each punches.list as punch}
            <PunchCard {punch} />
        {/each}
    </div>
</main>
