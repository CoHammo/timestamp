<script lang="ts">
    import Tags from "@lucide/svelte/icons/tags";
    import { Modal, TagCard } from "./index";
    import { Punch } from "../types.svelte";
    import { updatePunch, addPunch } from "$lib/commands";

    let modal: Modal | undefined = $state();
    let punch: Punch | undefined = $state();
    let index: number = $state(-1);

    export function open(p: Punch, listIndex: number) {
        punch = p;
        index = listIndex;
        modal?.open();
    }

    export function close() {
        punch = undefined;
        index = -1;
        modal?.close();
    }
</script>

<Modal bind:this={modal}>
    <div class="flex flex-col gap-2 p-2">
        <h2 class="font-bold text-center px-8 mb-1 mt-2 text-[1.3rem]">
            {punch?.start.toLocaleDateString([], { weekday: "long" })},
            {punch?.start.toLocaleDateString([], {
                month: "long",
                day: "numeric",
            })}
        </h2>
        <div class="flex flex-wrap gap-2 text-center text-[1.06rem]">
            <div class="flex flex-col bg-slate-300/80 rounded p-2 flex-1">
                <span>
                    {punch?.start.toLocaleDateString([])}
                </span>
                <span>
                    {punch?.start.toLocaleTimeString([], {
                        hour: "numeric",
                        minute: "2-digit",
                    })}
                </span>
            </div>
            <div
                class="flex flex-col bg-slate-300/80 rounded p-2 flex-1 justify-center items-center"
            >
                {#if punch?.end != null}
                    <span>
                        {punch?.end?.toLocaleDateString([])}
                    </span>
                    <span>
                        {punch?.end?.toLocaleTimeString([], {
                            hour: "numeric",
                            minute: "2-digit",
                        })}
                    </span>
                {:else}
                    <span class="font-bold">
                        {punch?.dDelta.hours}h
                        {punch?.dDelta.minutes}m
                        {punch?.dDelta.seconds}s
                    </span>
                {/if}
            </div>
        </div>
        {#if punch?.end != null}
            <div
                class="bg-slate-300/80 rounded p-2 text-center font-bold text-[1.2rem]"
            >
                {punch?.dDelta.hours}h
                {punch?.dDelta.minutes}m
                {punch?.dDelta.seconds}s
            </div>
        {/if}
        <div class="flex flex-col">
            <label for="notes" class="text-[1.1rem] mb-0.5">Notes</label>
            <textarea
                name="notes"
                id="notebox"
                rows="4"
                value={punch?.notes}
                placeholder="Notes here..."
                class="bg-slate-200/60 p-2 rounded focus:bg-slate-300/60 focus:outline-none transition-all duration-300"
            ></textarea>
        </div>
        <button class="flex gap-1 p-1 my-2 hover:cursor-pointer">
            <div class="mr-1">
                <Tags size={22} color="blue" />
            </div>
            <div class="flex flex-wrap">
                {#each punch?.tags as tag}
                    <TagCard {tag} />
                {/each}
                {#if punch?.tags == undefined}
                    <span class="text-gray-600">Tags</span>
                {/if}
            </div>
        </button>
        <button
            class="btn outline-none border-none bg-blue-600 rounded text-lg text-white h-8 m-0"
            onclick={async () => {
                if (punch) {
                    if (punch.id == 0) {
                        await addPunch(punch);
                    } else {
                        await updatePunch(punch, index);
                    }
                }
            }}
        >
            Save
        </button>
    </div>
</Modal>
