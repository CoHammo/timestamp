<script lang="ts">
    import { PunchCard, type Punch } from "$lib";
    import { onMount } from "svelte";
    import {
        jobs,
        punches,
        state,
        init,
        correctPunch,
        getPunches,
    } from "./state.svelte";
    import Menu from "@lucide/svelte/icons/menu";
    import Settings from "@lucide/svelte/icons/settings";
    import { invoke } from "@tauri-apps/api/core";

    onMount(async () => {
        await init();
    });

    let clockedIn = $derived(punches.list[0]?.end == undefined);
</script>

<main class="h-screen flex flex-col">
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
                class="btn border-none bg-none hover:bg-white/20 px-3 rounded-md"
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
    <div class="flex flex-col gap-2 p-2 overflow-y-auto">
        {#each punches.list as punch (punch.id)}
            <PunchCard {punch} />
        {/each}
    </div>

    <!-- Bottom Buttons -->
    <div class="bg-amber-200 mt-auto p-2">
        <button
            class="btn border-none {clockedIn
                ? 'bg-red-500'
                : 'bg-blue-600'} rounded-md text-white text-lg w-full"
            onclick={async () => {
                try {
                    if (clockedIn) {
                        let punch: Punch = await invoke("clock_out", {
                            jobId: state.state?.job.id,
                        });
                        correctPunch(punch);
                        punches.list[0] = punch;
                    } else {
                        let punch: Punch = await invoke("clock_in", {
                            jobId: state.state?.job.id,
                        });
                        correctPunch(punch);
                        punches.list.splice(0, 0, punch);
                    }
                } catch (e) {
                    console.log(e);
                }
            }}>{clockedIn ? "Clock Out" : "Clock In"}</button
        >
    </div>
</main>
