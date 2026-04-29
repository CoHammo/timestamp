import { SvelteMap } from "svelte/reactivity";
import { Job, State, Punch, Tag } from "./types";
import { readable } from "svelte/store";

export const timer = readable(true, (set) => {
  let change = false;
  set(change);
  const id = setInterval(() => {
    change = !change;
    set(change);
  }, 1000);
  return () => clearInterval(id);
});

export const jobs: { list: Job[] } = $state({ list: [] });
export const punches: { list: Punch[] } = $state({ list: [] });
export const appState: { state: State | undefined } = $state({
  state: undefined,
});
export const tags: { map: SvelteMap<number, Tag> } = $state({
  map: new SvelteMap<number, Tag>(),
});
