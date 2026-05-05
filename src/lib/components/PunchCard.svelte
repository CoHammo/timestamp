<script lang="ts">
    import { TagCard } from "./index";
    import { Punch, type Delta } from "../types";
    import Tags from "@lucide/svelte/icons/tags";
    import Trash from "@lucide/svelte/icons/trash-2";
    import { getContext } from "svelte";
    import { tags, timer } from "../state.svelte";
    import { MediaQuery } from "svelte/reactivity";
    import { deletePunch } from "$lib/commands";
    import { slide } from "svelte/transition";

    let { punch, listIndex }: { punch: Punch; listIndex: number } = $props();

    const editPunch: (punch: Punch, listIndex: number) => void =
        getContext("editPunch");

    $effect(() => {
        delta = punch.getDelta();
        if (punch.end === undefined) {
            let unsub = timer.subscribe(() => {
                delta = punch.getDelta();
            });
            return () => unsub();
        }
    });
    let delta: Delta = $state({
        hours: 0,
        minutes: 0,
        seconds: 0,
        str: "00:00:00",
    });

    const large = new MediaQuery("min-width: 400px");
    const larger = new MediaQuery("min-width: 500px");
</script>

<div class="flex flex-row rounded-md overflow-hidden min-w-52">
    <button
        onclick={() => editPunch(punch, listIndex)}
        class="grid grid-cols-[auto_1fr] grid-rows-[auto_1fr_auto] flex-1 {punch.end ==
        undefined
            ? 'bg-blue-200'
            : 'bg-slate-300/80'} px-3 py-2 hover:cursor-pointer focus:outline-none transition-all duration-300"
    >
        <!-- Date -->
        <h2 class="flex text-[1.2rem] font-medium justify-start mr-1">
            {punch.start.toLocaleDateString([], { weekday: "long" })},
            {punch.start.toLocaleDateString([], {
                month: "short",
                day: "numeric",
            })}
        </h2>

        <!-- Duration -->
        {#if large.current}
            <!-- Spread Out -->
            <div
                class="row-span-2 flex flex-row items-center justify-end ml-1 text-[1.35rem] gap-1"
            >
                {#if delta.hours === 0 && delta.minutes === 0}
                    <span class="font-semibold">{delta.seconds}s</span>
                {:else if delta.hours === 0 && delta.minutes !== 0}
                    <span class="font-semibold">{delta.minutes}m</span>
                    <span>{delta.seconds}s</span>
                {:else}
                    <span class="font-semibold">{delta.hours}h</span>
                    <span>{delta.minutes}m</span>
                {/if}
            </div>
        {:else}
            <!-- Compact -->
            <div
                class="row-span-2 flex flex-col items-end justify-center ml-0.5"
            >
                {#if delta.hours === 0 && delta.minutes === 0}
                    <span class="text-[1.35rem] font-semibold"
                        >{delta.seconds}s</span
                    >
                {:else if delta.hours === 0 && delta.minutes !== 0}
                    <span class="text-[1.35rem] font-semibold"
                        >{delta.minutes}m</span
                    >
                    <span class="text-[1.1rem] -mt-1">{delta.seconds}s</span>
                {:else}
                    <span class="text-[1.35rem] font-semibold"
                        >{delta.hours}h</span
                    >
                    <span class="text-[1.1rem] -mt-1">{delta.minutes}m</span>
                {/if}
            </div>
        {/if}

        <!-- Time and Notes -->
        <div class="truncate flex flex-row">
            <!-- Start Time -->
            <span class="text-[1.05rem]"
                >{punch.start.toLocaleTimeString([], {
                    hour: "numeric",
                    minute: "numeric",
                })}</span
            >
            {#if large.current && punch.end !== undefined}
                <span class="mx-0.5">-</span>
                <span class="text-[1.05rem]"
                    >{punch.end?.toLocaleTimeString([], {
                        hour: "numeric",
                        minute: "numeric",
                    })}</span
                >
            {/if}

            <!-- Notes -->
            {#if punch.notes !== ""}
                <div class="flex flex-row truncate">
                    <span class="mx-1">-</span>
                    <div class="truncate">{punch.notes}</div>
                </div>
            {/if}
        </div>

        <!-- Tags -->
        {#if punch.tags.length > 0}
            <div class="col-span-2 flex items-center gap-2 mt-1 mb-0.5">
                <div>
                    <Tags size={23} color="blue" />
                </div>
                <div class="flex overflow-x-auto gap-1">
                    {#each punch.tags as tag}
                        <TagCard tag={tags.map.get(tag)} />
                    {/each}
                </div>
            </div>
        {/if}
    </button>

    {#if larger.current}
        <button
            transition:slide={{ axis: "x" }}
            onclick={() => deletePunch(punch.id, listIndex)}
            class="bg-red-300 w-14 flex justify-center items-center hover:cursor-pointer"
        >
            <div>
                <Trash size={28} />
            </div>
        </button>
    {/if}
</div>
