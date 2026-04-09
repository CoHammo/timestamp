<script lang="ts">
    import Tags from "@lucide/svelte/icons/tags";
    import { Modal, type Punch, TagCard } from "./index";

    // let { punch }: { punch: Punch } = $props();

    let punch: Punch | undefined = $state();
    let modal: Modal | undefined = $state();

    export function open(p: Punch) {
        punch = p;
        modal?.open();
    }

    export function close() {
        punch = undefined;
        modal?.close();
    }
</script>

<Modal bind:this={modal}>
    <div class="flex flex-col gap-2 p-3">
        <h2 class="text-lg font-bold text-center px-8 mb-2">
            {punch?.start.toLocaleDateString([], { weekday: "long" })},
            {punch?.start.toLocaleDateString([], {
                month: "long",
                day: "numeric",
            })}
        </h2>
        <div class="flex gap-2 text-center">
            <div class="flex flex-col bg-slate-300/90 rounded p-2 flex-1">
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
            <div class="flex flex-col bg-slate-300/90 rounded p-2 flex-1">
                <span>
                    {punch?.end?.toLocaleDateString([])}
                </span>
                <span>
                    {punch?.end?.toLocaleTimeString([], {
                        hour: "numeric",
                        minute: "2-digit",
                    })}
                </span>
            </div>
        </div>
        <textarea
            name="notes"
            id="notebox"
            rows="3"
            value={punch?.notes}
            placeholder="Notes here..."
            class="bg-slate-100 p-2 rounded outline focus:outline focus:outline-offset-0 focus:outline-black focus:bg-slate-200 transition-all duration-300"
        ></textarea>
        <button class="flex gap-1 p-1 hover:cursor-pointer">
            <Tags size={22} color="blue" />
            {#each punch?.tags as tag}
                <TagCard {tag} />
            {/each}
        </button>
    </div>
</Modal>
