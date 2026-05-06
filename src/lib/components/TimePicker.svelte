<script lang="ts">
    import { SvelteDate } from "svelte/reactivity";

    let chosenTime: SvelteDate | undefined = $state();
    let isAm: boolean = $state(true);
    let onChange: ((dt: Date) => void) | undefined = $state();
    let chosenHour: number = $state(0);

    export function open(dt: Date, onTimeChange?: (dt: Date) => void) {
        if (dt) {
            chosenTime = new SvelteDate(dt);
        }
        isAm = true;
        onChange = onTimeChange;
    }

    export function close() {
        isAm = true;
        chosenTime = undefined;
        onChange = undefined;
    }

    function changeHour() {
        let toSet: number = chosenHour;
        if (isAm && chosenHour === 12) {
            toSet = 0;
        } else if (!isAm && chosenHour !== 12) {
            toSet = chosenHour + 12;
        }
        chosenTime?.setHours(toSet);
    }
</script>

<div class="flex h-57 p-1">
    <div class="flex flex-col overflow-y-auto no-scrollbar text-[1.2rem]">
        {#each { length: 12 }, index}
            {@const hour = index + 1}
            {@const chosen = chosenTime?.getHours() === hour}
            <button
                onclick={() => {
                    if (!chosenTime) {
                        chosenTime = new SvelteDate();
                    }
                    chosenHour = hour;
                    changeHour();
                    onChange?.(new Date(chosenTime));
                }}
                class="min-w-11 min-h-11 hover:cursor-pointer rounded transition-all {chosen
                    ? 'bg-blue-600 text-white font-bold'
                    : 'hover:bg-gray-300'}">{hour}</button
            >
        {/each}
    </div>
    <div class="flex flex-col overflow-y-auto no-scrollbar text-[1.3rem]">
        {#each { length: 60 }, minute}
            {@const chosen = chosenTime?.getMinutes() === minute}
            <button
                onclick={() => {
                    if (!chosenTime) {
                        chosenTime = new SvelteDate();
                    }
                    chosenTime.setMinutes(minute);
                    onChange?.(new Date(chosenTime));
                }}
                class="min-w-11 min-h-11 hover:cursor-pointer rounded transition-all {chosen
                    ? 'bg-blue-600 text-white font-bold'
                    : 'hover:bg-gray-300'}"
                >{minute.toString().padStart(2, "0")}</button
            >
        {/each}
    </div>
    <div class="flex flex-col text-[1.3rem]">
        <button
            onclick={() => {
                isAm = true;
                changeHour();
                onChange?.(new Date(chosenTime!));
            }}
            class="min-w-11 min-h-11 hover:cursor-pointer rounded transition-all {isAm
                ? 'bg-blue-600 text-white font-bold'
                : 'hover:bg-gray-300'}">AM</button
        >
        <button
            onclick={() => {
                isAm = false;
                changeHour();
                onChange?.(new Date(chosenTime!));
            }}
            class="min-w-11 min-h-11 hover:cursor-pointer rounded transition-all {!isAm
                ? 'bg-blue-600 text-white font-bold'
                : 'hover:bg-gray-300'}">PM</button
        >
    </div>
</div>
