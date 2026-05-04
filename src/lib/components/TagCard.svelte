<script lang="ts">
    import { Tag } from "../types";
    let { tag }: { tag: Tag | undefined } = $props();
</script>

<div class="bg-white rounded-md">
    <div
        class="bg-blue-600/50 rounded px-1.5 py-px font-semibold text-[0.95rem]"
    >
        {tag?.name ?? "BAD TAG"}
    </div>
</div>
