<script lang="ts">
    import LeftArrow from "@lucide/svelte/icons/arrow-left";
    import RightArrow from "@lucide/svelte/icons/arrow-right";
    import { SvelteDate } from "svelte/reactivity";

    let maxDate: SvelteDate = $state(new SvelteDate());
    let minDate: SvelteDate | undefined = $state();
    let chosenDate: SvelteDate | undefined = $state();
    let viewDate: SvelteDate = $state(new SvelteDate());
    let monthDays: number = $state(0);
    let postDays: number = $state(0);
    let canDelete: boolean = $state(false);
    let onChoose: ((date: Date | undefined) => void) | undefined =
        $state(undefined);

    function changeView(d?: Date, addToMonth?: number) {
        if (d) {
            viewDate = new SvelteDate(d);
        } else if (addToMonth) {
            viewDate.setMonth(viewDate.getMonth() + addToMonth);
        }
        viewDate.setDate(1);
        let lastMonthDate = new Date(
            viewDate.getFullYear(),
            viewDate.getMonth() + 1,
            0,
        );
        monthDays = lastMonthDate.getDate();
        postDays = 6 - lastMonthDate.getDay();
    }

    export function open(
        d?: Date,
        max: Date = new Date(),
        min?: Date,
        onPick?: (date: Date | undefined) => void,
        canUnpick: boolean = false,
    ) {
        maxDate = new SvelteDate(max);
        maxDate.setHours(0, 0, 0, 0);
        minDate = new SvelteDate(min);
        minDate.setHours(0, 0, 0, 0);
        onChoose = onPick;
        canDelete = canUnpick;
        if (d) {
            chosenDate = new SvelteDate(d);
            changeView(d);
        } else {
            changeView(new Date());
        }
    }

    export function close() {
        maxDate = new SvelteDate();
        minDate = undefined;
        chosenDate = undefined;
        viewDate = new SvelteDate();
        monthDays = 0;
        postDays = 0;
        canDelete = false;
        onChoose = undefined;
    }
</script>

{#key chosenDate}
    <div class="grid grid-cols-[auto_1fr_auto] p-1">
        <button
            onclick={() => changeView(undefined, -1)}
            class="btn btn-circle border-none bg-none hover:bg-gray-200"
            ><LeftArrow size={20} /></button
        >
        <h3
            class="flex px-2 justify-center items-center text-[1.2rem] font-bold gap-2"
        >
            <span>
                {viewDate.toLocaleString("default", { month: "long" })}
            </span>
            <span>
                {viewDate.toLocaleString("default", { year: "numeric" })}
            </span>
        </h3>
        <button
            onclick={() => changeView(undefined, 1)}
            class="btn btn-circle border-none bg-none hover:bg-gray-200"
            ><RightArrow size={20} /></button
        >
        <div class="grid grid-cols-7 col-span-3">
            <!-- Days of Week -->
            {#each ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] as weekDay}
                <span class="text-gray-600 text-[0.95rem]">{weekDay}</span>
            {/each}
            <div class="border-t border-gray-500 col-span-7 mt-1"></div>

            <!-- Preceding Days -->
            {#each { length: viewDate.getDay() }}
                <div class="h-9 w-9"></div>
            {/each}

            <!-- Month Days -->
            {#each { length: monthDays }, num}
                {@const day = num + 1}
                {@const chosen =
                    chosenDate?.getFullYear() === viewDate.getFullYear() &&
                    chosenDate.getMonth() === viewDate.getMonth() &&
                    chosenDate.getDate() === day}
                {@const notChoosable =
                    new Date(viewDate.getFullYear(), viewDate.getMonth(), day) >
                        maxDate ||
                    (minDate &&
                        new Date(
                            viewDate.getFullYear(),
                            viewDate.getMonth(),
                            day,
                        ) < minDate)}
                {#if notChoosable}
                    <div class="h-9 w-9 text-gray-400 content-center">
                        {day}
                    </div>
                {:else}
                    <button
                        onclick={() => {
                            if (canDelete && chosenDate && chosen) {
                                chosenDate = undefined;
                                onChoose?.(undefined);
                            } else if (chosenDate) {
                                chosenDate.setFullYear(viewDate.getFullYear());
                                chosenDate.setMonth(viewDate.getMonth());
                                chosenDate.setDate(day);
                                onChoose?.(new Date(chosenDate));
                            } else {
                                chosenDate = new SvelteDate();
                                onChoose?.(new Date(chosenDate));
                            }
                        }}
                        class="h-9 w-9 hover:cursor-pointer rounded transition-all {chosen
                            ? 'bg-slate-800 text-white font-bold'
                            : 'hover:bg-gray-300'}">{day}</button
                    >
                {/if}
            {/each}

            <!-- Post Days -->
            {#each { length: postDays }}
                <div class="h-9 w-9"></div>
            {/each}
        </div>
    </div>
{/key}
