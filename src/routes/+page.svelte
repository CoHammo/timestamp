<script lang="ts">
    import { PunchCard } from "$lib/components";
    import { Punch } from "$lib/types";
    import { clockIn, clockOut } from "$lib/commands";
    import { appState, punches } from "$lib/state.svelte";
    import Menu from "@lucide/svelte/icons/menu";
    import Settings from "@lucide/svelte/icons/settings";
    import Play from "@lucide/svelte/icons/play";
    import Stop from "@lucide/svelte/icons/square";
    import AddClock from "@lucide/svelte/icons/clock-plus";
    import { getContext } from "svelte";

    const editPunch: (punch: Punch, listIndex: number) => void =
        getContext("editPunch");
</script>

<main class="h-screen flex flex-col">
    <!-- App Bar -->
    <div
        class="w-full flex bg-blue-600 justify-between text-white px-2 py-2 min-h-18 items-center gap-1"
    >
        <div class="flex items-center gap-0.5 flex-1">
            <button
                class="btn btn-circle border-none bg-none hover:bg-white/20 h-10 w-10"
            >
                <Menu />
            </button>
            <button
                class="border-none bg-none hover:bg-white/20 min-h-10 px-2 py-1 rounded-md flex-1 justify-start transition-all duration-200 hover:cursor-pointer"
            >
                <h1 class="text-[1.4rem]/7 font-bold">
                    {appState.state?.job.name}
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
    <div
        class="flex flex-col gap-2 p-2 overflow-y-auto min-[600px]:min-w-150 min-[600px]:self-center"
    >
        {#each { length: punches.list.length } as _, index}
            {@const i = punches.list.length - 1 - index}
            {@const punch = punches.list[i]}
            <PunchCard {punch} listIndex={i} />
        {/each}
    </div>

    <!-- Bottom Buttons -->
    <div class="flex flex-wrap mt-auto p-1.5 gap-1.5">
        <button
            class="btn flex-1 bg-blue-600 text-white text-[1rem]/4 text-nowrap border-none rounded gap-2 focus:border-none focus:outline-none"
            onclick={() => editPunch(new Punch(appState.state!.job.id), -1)}
        >
            <div>
                <AddClock size={22} />
            </div>
            Add Entry
        </button>

        <button
            class="btn border-none flex-1 {appState.state?.job.clocked_in
                ? 'bg-red-500'
                : 'bg-green-600'} rounded text-white text-[1rem]/4 text-nowrap"
            onclick={async () => {
                if (appState.state?.job.clocked_in) {
                    await clockOut();
                } else {
                    await clockIn();
                }
            }}
        >
            {#if appState.state?.job.clocked_in}
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

<!-- <div></div> -->
