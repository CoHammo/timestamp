<script lang="ts">
    import { type Punch } from "./index";

    let { punch }: { punch: Punch } = $props();
</script>

<button class="flex items-center btn border-none bg-gray-300 rounded-md">
    <div class="flex flex-col">{punch.start.toDateString().slice(0, 3)}</div>
</button>
