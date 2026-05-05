<script lang="ts">
    import LeftArrow from "@lucide/svelte/icons/arrow-left";
    import RightArrow from "@lucide/svelte/icons/arrow-right";
    import { onMount } from "svelte";

    let date: Date = $state(new Date());
    let chosenDate: Date = $state(new Date());
    let viewDate: Date = $state(new Date());
    let monthDays: number[] = $state([]);

    export function open(d: Date = new Date()) {
        date = d;
        chosenDate = new Date(date);
        viewDate = new Date(date);
        let first = new Date(
            viewDate.getFullYear(),
            viewDate.getMonth(),
            1,
        ).getDay();
        console.log(first);
    }
</script>

<div class="grid grid-cols-[auto_1fr_auto] p-1">
    <button class="btn btn-circle border-none bg-none hover:bg-gray-200"
        ><LeftArrow size={20} /></button
    >
    <h3 class="flex px-2 justify-center items-center text-[1.2rem] font-bold">
        {viewDate.toLocaleString("default", { month: "long", year: "numeric" })}
    </h3>
    <button class="btn btn-circle border-none bg-none hover:bg-gray-200"
        ><RightArrow size={20} /></button
    >
    <div
        class="grid grid-cols-7 grid-rows-[auto_auto_repeat(6,1fr)] col-span-3"
    >
        {#each ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] as weekDay}
            <span class="text-gray-600 text-[0.95rem]">{weekDay}</span>
        {/each}
        <div class="border-t border-gray-500 col-span-7 mt-1"></div>
        {#each { length: 42 }, day}
            <button
                class="h-10 w-10 hover:cursor-pointer hover:bg-gray-200 rounded transition-all p-0 m-0"
                >{day}</button
            >
        {/each}
    </div>
</div>
