<script lang="ts">
    let { tag }: { tag: string } = $props();
</script>

<div class="bg-white rounded-md">
    <div
        class="bg-indigo-400/50 rounded px-1.5 py-px font-semibold text-[0.9rem]"
    >
        {tag}
    </div>
</div>
