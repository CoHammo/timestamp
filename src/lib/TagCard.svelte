<script lang="ts">
    let { tag }: { tag: string } = $props();
</script>

<div class="bg-gray-300 rounded-md">
    <div
        class="bg-indigo-600/30 rounded-md px-1.5 py-px font-semibold text-[0.9rem]"
    >
        {tag}
    </div>
</div>
