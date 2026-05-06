<script lang="ts">
    import Tags from "@lucide/svelte/icons/tags";
    import RightArrow from "@lucide/svelte/icons/arrow-right";
    import Refresh from "@lucide/svelte/icons/rotate-ccw";
    import { DatePicker, TagCard, TimePicker } from "./index";
    import "cally";
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
    let currentEnd: Date | undefined = $state();
    let tagged: Tag[] = $state([]);
    let availableTags: Tag[] = $state([]);
    let index: number = $state(-1);
    let delta: Delta = $state({
        hours: 0,
        minutes: 0,
        seconds: 0,
        str: "00:00:00",
    });
    let unsub: (() => void) | undefined;
    let changed = $state(false);

    function setPunch() {
        if (orig) {
            punch = orig.clone();
            currentEnd = undefined;
            delta = punch.getDelta();
            tagged = punch.tags.map((id) => tags.map.get(id)!);
            availableTags = tags.map
                .values()
                .filter((t) => !punch?.tags.includes(t.id))
                .toArray();
            stopTimer();
            startTimer();
            checkChange();
        }
    }

    function startTimer() {
        if (punch && punch.end === undefined) {
            currentEnd = new Date();
            unsub = timer.subscribe(() => {
                delta = punch!.getDelta();
                currentEnd = new Date();
            });
        }
    }

    function checkChange() {
        if (orig && punch) {
            changed = !orig.equals(punch);
        }
    }

    function stopTimer() {
        unsub?.();
        unsub = undefined;
        currentEnd = undefined;
    }

    export function open(p: Punch, listIndex: number) {
        orig = p;
        setPunch();
        index = listIndex;
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
            stopTimer();
            changed = false;
        }, 100);
    }

    function changeNotes(notes: string) {
        if (punch) {
            punch.notes = notes;
            checkChange();
        }
    }

    function removeTag(index: number) {
        if (punch) {
            availableTags.push(tagged[index]);
            tagged.splice(index, 1);
            punch?.tags.splice(index, 1);
            checkChange();
        }
    }

    function addTag(index: number) {
        if (punch) {
            let tag = availableTags[index];
            availableTags.splice(index, 1);
            tagged.push(tag);
            punch?.tags.push(tag.id);
            checkChange();
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

    let startDatePicker: DatePicker | undefined = $state();
    let endDatePicker: DatePicker | undefined = $state();
    let startTimePicker: TimePicker | undefined = $state();
    let endTimePicker: TimePicker | undefined = $state();
    let startChanged = $state(false);
    let endChanged = $state(false);
</script>

<Modal bind:this={modal}>
    <button
        disabled={!changed}
        onclick={() => setPunch()}
        class="btn btn-circle border-none w-10 h-10 absolute top-1 left-1 bg-none hover:bg-gray-200"
    >
        <Refresh size={23} color={!changed ? "gray" : "blue"} />
    </button>
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
                class="flex flex-1 bg-slate-300/80 rounded overflow-hidden min-w-48 text-[1.1rem]"
            >
                <!-- Start Date Button -->
                {#key startChanged}
                    <button
                        popovertarget="start-date-popover"
                        style="anchor-name:--start-date-anchor"
                        class="border-r py-1.5 px-3 hover:cursor-pointer hover:bg-slate-400/80 transition-all w-26"
                    >
                        {punch?.start.toLocaleDateString([])}
                    </button>
                {/key}
                <!-- Start DatePicker Popover -->
                <div
                    popover
                    ontoggle={(e) => {
                        if (e.newState === "open" && punch) {
                            startDatePicker?.open(
                                punch?.start,
                                currentEnd,
                                undefined,
                                (d) => {
                                    if (orig && punch && d) {
                                        punch.start = d;
                                        startChanged = !startChanged;
                                        delta = punch.getDelta();
                                        checkChange();
                                    }
                                },
                            );
                        } else {
                            setTimeout(() => startDatePicker?.close(), 150);
                        }
                    }}
                    id="start-date-popover"
                    style="position-anchor:--start-date-anchor"
                    class="dropdown bg-white rounded border-none outline-none content-center shadow-[1px_1px_4px]"
                >
                    <DatePicker bind:this={startDatePicker} />
                </div>

                <!-- Start Time Button -->
                {#key startChanged}
                    <button
                        popovertarget="start-time-popover"
                        style="anchor-name:--start-time-anchor"
                        class="flex-1 py-1.5 px-3 min-w-28 content-center hover:bg-slate-400/80 transition-all hover:cursor-pointer"
                    >
                        {punch?.start.toLocaleTimeString([], {
                            hour: "numeric",
                            minute: "2-digit",
                        })}
                    </button>
                {/key}
                <!-- Start TimePicker Popover -->
                <div
                    popover
                    ontoggle={(e) => {
                        if (e.newState === "open" && punch) {
                            startTimePicker?.open(punch?.start, (dt) => {
                                if (punch) {
                                    punch.start = dt;
                                    startChanged = !startChanged;
                                    checkChange();
                                }
                            });
                        } else {
                            startTimePicker?.close();
                        }
                    }}
                    id="start-time-popover"
                    style="position-anchor:--start-time-anchor"
                    class="dropdown bg-white rounded border-none outline-none content-center shadow-[1px_1px_4px]"
                >
                    <TimePicker bind:this={startTimePicker} />
                </div>
            </div>

            {#if !currentEnd && punch && punch.end}
                <!-- End Box -->
                <div
                    class="flex flex-1 bg-slate-300/80 rounded overflow-hidden min-w-48 text-[1.1rem]"
                >
                    <!-- End Date Button -->
                    {#key endChanged}
                        <button
                            popovertarget="end-date-popover"
                            style="anchor-name:--end-date-anchor"
                            class="border-r py-1.5 px-3 hover:cursor-pointer hover:bg-slate-400/80 content-center transition-all w-26"
                        >
                            {punch.end.toLocaleDateString([])}
                        </button>
                    {/key}
                    <!-- End DatePicker Popover -->
                    <div
                        popover
                        ontoggle={(e) => {
                            if (e.newState === "open" && punch?.end) {
                                endDatePicker?.open(
                                    punch.end,
                                    undefined,
                                    punch.start,
                                    (dt) => {
                                        if (punch) {
                                            punch.end = dt;
                                            endChanged = !endChanged;
                                            delta = punch.getDelta();
                                            checkChange();
                                            stopTimer();
                                            startTimer();
                                        }
                                    },
                                    index === punches.list.length - 1,
                                );
                            } else {
                                setTimeout(() => endDatePicker?.close(), 150);
                            }
                        }}
                        id="end-date-popover"
                        style="position-anchor:--end-date-anchor"
                        class="dropdown bg-white rounded border-none outline-none shadow-[1px_1px_4px]"
                    >
                        <DatePicker bind:this={endDatePicker} />
                    </div>

                    <!-- End Time Button -->
                    {#key endChanged}
                        <button
                            popovertarget="end-time-popover"
                            style="anchor-name:--end-time-anchor"
                            class="flex-1 py-1.5 px-3 min-w-28 content-center hover:bg-slate-400/80 hover:cursor-pointer transition-all"
                        >
                            {punch.end.toLocaleTimeString([], {
                                hour: "numeric",
                                minute: "2-digit",
                            })}
                        </button>
                    {/key}
                    <!-- End TimePicker Popover -->
                    <div
                        popover
                        ontoggle={(e) => {
                            if (e.newState === "open" && punch?.end) {
                                endTimePicker?.open(punch.end, (dt) => {
                                    if (punch) {
                                        punch.end = dt;
                                        endChanged = !endChanged;
                                        checkChange();
                                        stopTimer();
                                        startTimer();
                                    }
                                });
                            } else {
                                endTimePicker?.close();
                            }
                        }}
                        id="end-time-popover"
                        style="position-anchor:--end-time-anchor"
                        class="dropdown bg-white rounded border-none outline-none content-center shadow-[1px_1px_4px]"
                    >
                        <TimePicker bind:this={endTimePicker} />
                    </div>
                </div>
            {:else}
                <!-- Current End Date and Time -->
                <button
                    onclick={() => {
                        if (punch) {
                            punch.end = new Date();
                            checkChange();
                            stopTimer();
                            startTimer();
                        }
                    }}
                    class="flex flex-1 bg-slate-300/80 rounded overflow-hidden min-w-48 text-[1.1rem] text-gray-600 hover:bg-slate-400/80 hover:cursor-pointer transition-all"
                >
                    <!-- Current End Date -->
                    <div class="border-r py-1.5 px-3 content-center w-26">
                        {currentEnd?.toLocaleDateString([])}
                    </div>
                    <!-- Current End Time -->
                    <div class="flex-1 py-1.5 px-3 min-w-28 content-center">
                        {currentEnd?.toLocaleTimeString([], {
                            hour: "numeric",
                            minute: "2-digit",
                        })}
                    </div>
                </button>
            {/if}
        </div>

        <!-- Timer -->
        <div
            class="bg-slate-300/80 rounded p-1.5 text-center font-bold text-[1.2rem]"
        >
            {delta.hours}h
            {delta.minutes}m
            {delta.seconds}s
        </div>

        <!-- Notes Box -->
        <div class="flex flex-col">
            <label for="notes" class="text-[1.1rem] mb-0.5">Notes</label>
            <textarea
                name="notes"
                id="notebox"
                rows="4"
                value={punch?.notes}
                placeholder="Notes here..."
                class="bg-slate-200/80 p-2 placeholder:text-gray-700 rounded focus:bg-slate-300/70 focus:outline-none transition-all duration-300"
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

        <!-- Buttons -->
        <div class="flex flex-col gap-2">
            <!-- Save Button -->
            <button
                class="btn flex-1 outline-none border-none bg-blue-600 rounded duration-200 transition-all text-lg text-white h-8 m-0 disabled:bg-slate-400 disabled:text-gray-300"
                disabled={!changed}
                onclick={async () => await save()}
            >
                Save
            </button>

            <!-- Delete Button -->
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
