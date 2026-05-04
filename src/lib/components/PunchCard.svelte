<script lang="ts">
    import { TagCard } from "./index";
    import { Punch, type Delta } from "../types";
    import Tags from "@lucide/svelte/icons/tags";
    import Notes from "@lucide/svelte/icons/sticky-note";
    import { getContext } from "svelte";
    import { tags, timer } from "../state.svelte";

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
</script>

<button
    onclick={() => editPunch(punch, listIndex)}
    class="grid grid-cols-[auto_1fr] rounded {punch.end == undefined
        ? 'bg-green-300/80'
        : 'bg-slate-300/80'} px-3 py-2 hover:cursor-pointer focus:outline-none"
>
    <!-- Date -->
    <h2 class="flex text-[1.15rem] font-medium justify-start">
        {punch.start.toLocaleDateString([], { weekday: "short" })},
        {punch.start.toLocaleDateString([], {
            month: "short",
            day: "numeric",
        })}
    </h2>

    <!-- Duration -->
    <div class="row-span-2 flex flex-col items-end">
        {#if delta.hours === 0 && delta.minutes === 0}
            <span class="text-[1.35rem] font-semibold">{delta.seconds}s</span>
        {:else if delta.hours === 0 && delta.minutes !== 0}
            <span class="text-[1.35rem] font-semibold">{delta.minutes}m</span>
            <span class="text-[1rem] -mt-1">{delta.seconds}s</span>
        {:else}
            <span class="text-[1.35rem] font-semibold">{delta.hours}h</span>
            <span class="text-[1rem] -mt-1">{delta.minutes}m</span>
        {/if}
    </div>

    <!-- Time and Notes -->
    <div class="truncate flex flex-row">
        <!-- Time -->
        <span class="text-[1.05rem]"
            >{punch.start.toLocaleTimeString([], {
                hour: "numeric",
                minute: "numeric",
            })}</span
        >

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
        <div class="col-span-2 flex items-center gap-2 mt-1.5 mb-0.5">
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

<!-- <button
    class="flex flex-col {punch.end == undefined
        ? 'bg-green-300'
        : 'bg-slate-300/80'} rounded-md font-normal px-3 py-2 hover:cursor-pointer focus:outline-none min-w-0 overflow-hidden"
    onclick={() => editPunch(punch, listIndex)}
>
    <div class="flex items-center justify-between overflow-hidden">
        <div class="flex flex-col items-start gap-px overflow-hidden">
            <span class="text-[1.15rem] font-medium overflow-hidden"
                >{punch.start.toLocaleDateString([], { weekday: "short" })},
                {punch.start.toLocaleDateString([], {
                    month: "short",
                    day: "numeric",
                })}</span
            >
            <div class="flex flex-row overflow-hidden">
                <span class="text-[1.05rem] text-nowrap"
                    >{punch.start.toLocaleTimeString([], {
                        hour: "numeric",
                        minute: "numeric",
                    })}
                </span>
                {#if punch.notes !== ""}
                    <span class="mx-1 shrink-0">-</span>
                    <div class="truncate border">{punch.notes}</div>
                {/if}
            </div>
        </div>

        <div class="flex flex-col items-end ml-1">
            {#if delta.hours === 0 && delta.minutes === 0}
                <span class="text-[1.35rem] font-semibold"
                    >{delta.seconds}s</span
                >
            {:else if delta.hours === 0 && delta.minutes !== 0}
                <span class="text-[1.35rem] font-semibold"
                    >{delta.minutes}m</span
                >
                <span class="text-[1rem] -mt-1">{delta.seconds}s</span>
            {:else}
                <span class="text-[1.35rem] font-semibold">{delta.hours}h</span>
                <span class="text-[1rem] -mt-1">{delta.minutes}m</span>
            {/if}
        </div>
    </div>

    {#if punch.tags.length > 0}
        <div class="flex items-center gap-2 mt-1.5 mb-0.5">
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
</button> -->
