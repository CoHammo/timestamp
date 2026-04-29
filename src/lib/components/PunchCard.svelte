<script lang="ts">
    import { TagCard } from "./index";
    import { Punch, type Delta } from "../types";
    import Tags from "@lucide/svelte/icons/tags";
    import { getContext, untrack } from "svelte";
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
    class="flex flex-col {punch.end == undefined
        ? 'bg-blue-200'
        : 'bg-slate-300/80'} rounded-md font-normal px-4 py-2 hover:cursor-pointer focus:outline-none"
    onclick={() => editPunch(punch.clone(), listIndex)}
>
    <!-- Data -->
    <div class="flex items-center justify-between">
        <!-- Date and Time of Punch Entry -->
        <div class="flex flex-col items-start gap-px">
            <span class="text-[1.15rem] font-medium"
                >{punch.start.toLocaleDateString([], { weekday: "short" })},
                {punch.start.toLocaleDateString([], {
                    month: "short",
                    day: "numeric",
                })}</span
            >
            <span class="text-[1.05rem]"
                >{punch.start.toLocaleTimeString([], {
                    hour: "numeric",
                    minute: "numeric",
                })}
            </span>
        </div>

        <!-- Duration of Punch Entry -->
        <div class="flex flex-col items-end">
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

    <!-- Tags -->
    {#if punch.tags.length > 0}
        <div class="flex items-center gap-2 mt-3 mb-1">
            <div>
                <Tags size={22} color="blue" />
            </div>
            <div class="flex overflow-x-auto gap-1">
                {#each punch.tags as tag}
                    <TagCard tag={tags.map.get(tag)} />
                {/each}
            </div>
        </div>
    {/if}
</button>
