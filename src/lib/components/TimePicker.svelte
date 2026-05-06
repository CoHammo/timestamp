<script lang="ts">
</script>

<div class="flex max-h-52 p-1">
    <div class="flex flex-col overflow-y-auto no-scrollbar text-[1.3rem]">
        {#each { length: 12 }, index}
            {@const hour = index + 1}
            <button
                class="min-w-11 min-h-11 hover:bg-gray-300 hover:cursor-pointer rounded transition-all"
                >{hour}</button
            >
        {/each}
    </div>
    <div class="flex flex-col overflow-y-auto no-scrollbar text-[1.3rem]">
        {#each { length: 60 }, minute}
            <button class="min-w-11 min-h-11"
                >{minute.toString().padStart(2, "0")}</button
            >
        {/each}
    </div>
</div>
