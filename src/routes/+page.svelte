<script lang="ts">
    import { PunchCard, PunchEditor } from "$lib/components";
    import { type PunchType, Punch } from "$lib/types.svelte";
    import {
        jobs,
        punches,
        appState,
        clockedIn,
        init,
        toggleClockIn,
    } from "$lib/state_commands.svelte";
    import { onMount } from "svelte";
    import Menu from "@lucide/svelte/icons/menu";
    import Settings from "@lucide/svelte/icons/settings";
    import Play from "@lucide/svelte/icons/play";
    import Stop from "@lucide/svelte/icons/square";
    import AddClock from "@lucide/svelte/icons/clock-plus";

    onMount(async () => {
        await init();
    });

    let newPuncher: PunchEditor | undefined = $state();
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
    <div class="flex flex-col gap-2 p-2 overflow-y-auto">
        {#each punches.list as punch (punch.id)}
            <PunchCard {punch} />
        {/each}
    </div>

    <!-- Bottom Buttons -->
    <div class="flex flex-wrap mt-auto p-1.5 gap-1.5">
        <button
            class="btn flex-1 bg-blue-600 text-white text-[1rem]/4 text-nowrap border-none rounded-md gap-2 focus:border-none focus:outline-none"
            onclick={() =>
                newPuncher?.open(new Punch(appState.state?.job.id ?? 0))}
        >
            <div>
                <AddClock size={22} />
            </div>
            Add Entry
        </button>

        <button
            class="btn border-none flex-1 {clockedIn()
                ? 'bg-red-500'
                : 'bg-green-600'} rounded-md text-white text-[1rem]/4 text-nowrap"
            onclick={async () => await toggleClockIn()}
        >
            {#if clockedIn()}
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

<PunchEditor bind:this={newPuncher} />
