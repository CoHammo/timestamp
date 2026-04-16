<script lang="ts">
    let { children } = $props();

    export function open() {
        modal?.showModal();
    }

    export function close() {
        modal?.close();
    }

    let modal: HTMLDialogElement | undefined = $state();
</script>

<dialog bind:this={modal} class="modal">
    <div
        class="modal-box bg-white rounded-md p-0 min-h-18 max-w-140 duration-100"
    >
        <button
            class="btn btn-circle bg-none border-none w-10 h-10 hover:bg-gray-200 absolute right-1 top-1"
            onclick={close}>X</button
        >
        {@render children()}
    </div>
    <form method="dialog" class="modal-backdrop">
        <button>close</button>
    </form>
</dialog>
