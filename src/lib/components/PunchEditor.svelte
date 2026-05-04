<script lang="ts">
    import Tags from "@lucide/svelte/icons/tags";
    import Square from "@lucide/svelte/icons/square";
    import Play from "@lucide/svelte/icons/play";
    import { TagCard } from "./index";
    import {
        updatePunch,
        addPunch,
        deletePunch,
        clockOut,
        clockIn,
        reClockIn,
    } from "$lib/commands";
    import { Modal } from "./index";
    import { Punch, Tag, type Delta } from "../types";
    import { punches, tags, timer } from "../state.svelte";

    let modal: Modal | undefined = $state();
    let orig: Punch | undefined = $state();
    let punch: Punch | undefined = $state();
    let tagged: Tag[] = $state([]);
    let availableTags: Tag[] = $state([]);
    let index: number = $state(-1);
    let delta: Delta = $state({
        hours: 0,
        minutes: 0,
        seconds: 0,
        str: "00:00:00",
    });
    let unsub: () => void | undefined;
    let changed = $state(false);

    export function open(p: Punch, listIndex: number) {
        orig = p;
        punch = p.clone();
        delta = punch.getDelta();
        index = listIndex;
        tagged = punch.tags.map((id) => tags.map.get(id)!);
        availableTags = tags.map
            .values()
            .filter((t) => !punch?.tags.includes(t.id))
            .toArray();
        if (punch.end === undefined) {
            unsub = timer.subscribe(() => {
                delta = punch!.getDelta();
            });
        }
        changed = !orig.equals(punch);
        modal?.open();
    }

    export async function close() {
        modal?.close();
        setTimeout(() => {
            punch = undefined;
            index = -1;
            delta = { hours: 0, minutes: 0, seconds: 0, str: "00:00:00" };
            tagged = [];
            availableTags = [];
            unsub?.();
            changed = false;
        }, 100);
    }

    function changeNotes(notes: string) {
        if (orig && punch) {
            punch.notes = notes;
            changed = !orig.equals(punch);
        }
    }

    function removeTag(index: number) {
        if (orig && punch) {
            availableTags.push(tagged[index]);
            tagged.splice(index, 1);
            punch?.tags.splice(index, 1);
            changed = !orig.equals(punch);
        }
    }

    function addTag(index: number) {
        if (orig && punch) {
            let tag = availableTags[index];
            availableTags.splice(index, 1);
            tagged.push(tag);
            punch?.tags.push(tag.id);
            changed = !orig.equals(punch);
        }
    }

    async function save() {
        if (punch) {
            if (punch.id == 0) {
                await addPunch(punch);
            } else {
                await updatePunch(punch, index);
            }
            close();
        }
    }

    async function trash() {
        if (punch) {
            await deletePunch(punch.id, index);
        }
        close();
    }

    $effect(() => {
        const tagArray = [...tags.map.values()];
        console.log(JSON.stringify(tagArray));
    });
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
            <!-- Start Box -->
            <div
                class="flex flex-col bg-slate-300/80 rounded p-2 flex-1 min-w-32 justify-center"
            >
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

            <!-- End Box -->
            <div
                class="flex flex-col bg-slate-300/80 rounded p-2 flex-1 min-w-32 justify-center"
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
                        {delta.hours}h
                        {delta.minutes}m
                        {delta.seconds}s
                    </span>
                {/if}
            </div>
        </div>
        {#if punch?.end != null}
            <div
                class="bg-slate-300/80 rounded p-2 text-center font-bold text-[1.2rem]"
            >
                {delta.hours}h
                {delta.minutes}m
                {delta.seconds}s
            </div>
        {/if}

        <!-- Notes Box -->
        <div class="flex flex-col">
            <label for="notes" class="text-[1.1rem] mb-0.5">Notes</label>
            <textarea
                name="notes"
                id="notebox"
                rows="4"
                value={punch?.notes}
                placeholder="Notes here..."
                class="bg-slate-200/80 p-2 rounded focus:bg-slate-300/70 focus:outline-none transition-all duration-300"
                oninput={(e) => changeNotes(e.currentTarget.value)}
            ></textarea>
        </div>

        <!-- Tags Button -->
        <button
            popovertarget="tag-popover"
            style="anchor-name:--tags-anchor"
            class="flex gap-1 p-2 items-center hover:cursor-pointer hover:bg-slate-200/80 transition-all duration-200 rounded"
        >
            <div class="mr-1">
                <Tags size={26} color="blue" />
            </div>
            <div class="flex flex-wrap gap-1">
                {#each tagged as tag}
                    <TagCard {tag} />
                {/each}
                {#if tagged.length === 0}
                    <span class="text-gray-600">Tags</span>
                {/if}
            </div>
        </button>
        <div
            popover
            id="tag-popover"
            style="position-anchor:--tags-anchor"
            class="dropdown menu bg-white shadow-[0px_2px_4px] rounded transition-100 -top-11.5 left-8.5"
        >
            <div class="flex flex-col gap-3 max-w-120 py-1">
                <div class="flex flex-wrap gap-1">
                    {#each tagged as tag, index (tag.id)}
                        <button
                            onclick={() => removeTag(index)}
                            class="hover:cursor-pointer"
                        >
                            <TagCard {tag} />
                        </button>
                    {:else}
                        <span class="text-gray-600 text-[1rem]">Tags</span>
                    {/each}
                </div>
                <hr />
                <div class="flex gap-1">
                    <h3 class="text-[1rem] font-bold">Add:</h3>
                    <div class="flex flex-wrap gap-1">
                        {#each availableTags as tag, index}
                            <button
                                onclick={() => addTag(index)}
                                class="hover:cursor-pointer"
                            >
                                <TagCard {tag} />
                            </button>
                        {/each}
                    </div>
                </div>
            </div>
        </div>

        <!-- Save Button -->
        <div class="flex flex-col gap-2">
            <button
                class="btn flex-1 outline-none border-none bg-blue-600 rounded duration-200 transition-all text-lg text-white h-8 m-0 disabled:bg-slate-400 disabled:text-gray-300"
                disabled={!changed}
                onclick={async () => await save()}
            >
                Save
            </button>
            {#if punch && punch.id !== 0}
                <button
                    class="btn flex-1 outline-none border-none bg-red-500 rounded text-lg text-white h-8 m-0 min-w-28"
                    onclick={async () => await trash()}
                >
                    Delete
                </button>
            {/if}
        </div>
    </div>
</Modal>
