<script lang="ts">
    import Tags from "@lucide/svelte/icons/tags";
    import { Modal, TagCard } from "./index";
    import { Punch } from "../types.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { addOrEditPunch } from "$lib/state_commands.svelte";

    let modal: Modal | undefined = $state();
    let punch: Punch | undefined = $state();

    export function open(p: Punch) {
        punch = p;
        modal?.open();
    }

    export function close() {
        punch = undefined;
        modal?.close();
    }
</script>

<Modal bind:this={modal}>
    <div class="flex flex-col gap-2 p-2">
        <h2 class="font-bold text-center px-8 my-1 text-[1.2rem]">
            {punch?.start.toLocaleDateString([], { weekday: "long" })},
            {punch?.start.toLocaleDateString([], {
                month: "long",
                day: "numeric",
            })}
        </h2>
        <div class="flex gap-2 text-center text-[1.1rem]">
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
        <textarea
            name="notes"
            id="notebox"
            rows="4"
            value={punch?.notes}
            placeholder="Notes here..."
            class="bg-slate-100 p-2 rounded focus:bg-slate-200 focus:outline-none transition-all duration-300"
        ></textarea>
        <button class="flex gap-1 p-1 my-1 hover:cursor-pointer">
            <div>
                <Tags size={22} color="blue" />
            </div>
            <div class="flex flex-wrap">
                {#each punch?.tags as tag}
                    <TagCard {tag} />
                {/each}
                {#if punch?.tags == undefined}
                    <span class="text-gray-600">No Tags...</span>
                {/if}
            </div>
        </button>
        <button
            class="btn outline-none border-none bg-blue-600 rounded text-lg text-white h-8 m-0"
            onclick={async () => await addOrEditPunch(punch!)}
        >
            Save
        </button>
    </div>
</Modal>
