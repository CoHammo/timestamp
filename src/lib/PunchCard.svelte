<script lang="ts">
    import { onMount } from "svelte";
    import { type Punch, TagCard } from "./index";
    import Tags from "@lucide/svelte/icons/tags";

    let { punch }: { punch: Punch } = $props();

    function formatMs(ms: number) {
        // 1. Calculate total units
        const totalSeconds = Math.floor(ms / 1000);
        const totalMinutes = Math.floor(totalSeconds / 60);

        // 2. Extract remaining parts using the modulo operator (%)
        const seconds = totalSeconds % 60;
        const minutes = totalMinutes % 60;
        const hours = Math.floor(totalMinutes / 60);

        // 3. Helper to add leading zeros
        const pad = (num: number) => num.toString().padStart(2, "0");

        return {
            hours,
            minutes,
            seconds,
            str: `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`,
        };
    }

    let delta: {
        hours: number;
        minutes: number;
        seconds: number;
        str: string;
    } = $state({ hours: 0, minutes: 0, seconds: 0, str: "" });

    let intervalId = $state(0);

    $effect(() => {
        if (punch.end != undefined) {
            clearInterval(intervalId);
        }
    });

    onMount(() => {
        if (punch.end == undefined) {
            intervalId = setInterval(() => {
                delta = formatMs(Date.now() - punch.start.getTime());
            }, 1000);
        } else {
            delta = formatMs(punch.delta ?? 0);
        }
        return () => clearInterval(intervalId);
    });
</script>

<button
    class="flex flex-col {punch.end == undefined
        ? 'bg-blue-200'
        : 'bg-slate-300'} rounded-md font-normal px-4 py-2 hover:cursor-pointer"
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
                <span class="text-[1rem] -mt-0.5">{delta.seconds}s</span>
            {:else}
                <span class="text-[1.35rem] font-semibold">{delta.hours}h</span>
                <span class="text-[1rem] -mt-0.5">{delta.minutes}m</span>
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
