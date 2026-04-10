<script lang="ts">
    import { PunchCard, type PunchType, PunchEditor, Punch } from "$lib";
    import { onMount } from "svelte";
    import { jobs, punches, things, init, getPunches } from "./state.svelte";
    import Menu from "@lucide/svelte/icons/menu";
    import Settings from "@lucide/svelte/icons/settings";
    import Play from "@lucide/svelte/icons/play";
    import Stop from "@lucide/svelte/icons/square";
    import AddClock from "@lucide/svelte/icons/clock-plus";
    import { invoke } from "@tauri-apps/api/core";

    onMount(async () => {
        await init();
    });

    let clockedIn = $derived(punches.list[0]?.end == undefined);
    let modal: PunchEditor | undefined = $state();
</script>

<main class="h-screen flex flex-col">
    <!-- App Bar -->
    <div
        class="w-full flex bg-blue-600 justify-between text-white px-2 py-3 min-h-18 items-center"
    >
        <div class="flex items-center gap-0.5 flex-1">
            <button
                class="btn btn-circle border-none bg-none hover:bg-white/20 h-10 w-10"
            >
                <Menu />
            </button>
            <button
                class="btn border-none bg-none hover:bg-white/20 px-3 rounded-md flex-1 justify-start"
            >
                <h1 class="text-[1.3rem] font-bold">
                    {things.state?.job.name}
                </h1>
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
    <div class="mt-auto p-2 flex gap-2">
        <!-- Add Punch Entry Button -->
        <button
            class="btn bg-blue-600 flex-1 text-white border-none rounded-md text-[1rem]/4 gap-2 focus:border-none focus:outline-none"
            onclick={() => modal?.open(punches.list[0])}
        >
            <div>
                <AddClock size={22} />
            </div>
            Add Entry
        </button>

        <!-- Clock In/Out Button -->
        <button
            class="btn border-none {clockedIn
                ? 'bg-red-500'
                : 'bg-green-600'} rounded-md text-white text-[1rem]/4 flex-1"
            onclick={async () => {
                try {
                    if (clockedIn) {
                        let p: PunchType = await invoke("clock_out", {
                            jobId: things.state?.job.id,
                        });
                        punches.list[0].clearTimer();
                        punches.list[0] = new Punch(p);
                    } else {
                        let p: PunchType = await invoke("clock_in", {
                            jobId: things.state?.job.id,
                        });
                        punches.list.splice(0, 0, new Punch(p));
                    }
                } catch (e) {
                    console.log(e);
                }
            }}
        >
            {#if clockedIn}
                <div>
                    <Stop fill="white" size={22} />
                </div>
                Clock Out
            {:else}
                <div>
                    <Play fill="white" size={22} />
                </div>
                Clock In
            {/if}
        </button>
    </div>
</main>

<PunchEditor bind:this={modal} />

<!-- <Modal bind:this={modal}>
    <div>Hello There</div>
</Modal> -->
