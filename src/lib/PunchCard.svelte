<script lang="ts">
    import { onMount } from "svelte";
    import { Punch, TagCard } from "./index";
    import Tags from "@lucide/svelte/icons/tags";

    let { punch }: { punch: Punch } = $props();
</script>

<button
    class="flex flex-col {punch.end == undefined
        ? 'bg-blue-200'
        : 'bg-slate-300/80'} rounded-md font-normal px-4 py-2 hover:cursor-pointer"
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
            {#if punch.dDelta.hours === 0 && punch.dDelta.minutes === 0}
                <span class="text-[1.35rem] font-semibold"
                    >{punch.dDelta.seconds}s</span
                >
            {:else if punch.dDelta.hours === 0 && punch.dDelta.minutes !== 0}
                <span class="text-[1.35rem] font-semibold"
                    >{punch.dDelta.minutes}m</span
                >
                <span class="text-[1rem] -mt-1">{punch.dDelta.seconds}s</span>
            {:else}
                <span class="text-[1.35rem] font-semibold"
                    >{punch.dDelta.hours}h</span
                >
                <span class="text-[1rem] -mt-1">{punch.dDelta.minutes}m</span>
            {/if}
        </div>
    </div>

    <!-- Tags -->
    {#if punch.tags != undefined}
        <div class="mb-1 mt-2 flex items-center gap-2">
            <Tags size={20} color="blue" />
            <div class="flex overflow-x-auto gap-1">
                {#each punch.tags as tag}
                    <TagCard {tag} />
                {/each}
            </div>
        </div>
    {/if}
</button>
