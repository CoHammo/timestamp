import { SvelteMap } from "svelte/reactivity";
import { Job, State, Punch, Tag } from "./types";
import { readable } from "svelte/store";

export const timer = readable(true, (set, up) => {
  set(false);
  console.log("timing...");
  const id = setInterval(() => {
    up((prev) => {
      const next = !prev;
      return next;
    });
  }, 1000);
  return () => {
    clearInterval(id);
    console.log("timer stopped");
  };
});

export const jobs: { list: Job[] } = $state({ list: [] });
export const punches: { list: Punch[] } = $state({ list: [] });
export const appState: { state: State | undefined } = $state({
  state: undefined,
});
export const tags: { map: Map<number, Tag> } = $state({
  map: new Map<number, Tag>(),
});
