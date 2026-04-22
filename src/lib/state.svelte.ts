import { Job, type State, Punch } from "./types.svelte";

export const jobs: { list: Job[] } = $state({ list: [] });
export const punches: { list: Punch[] } = $state({ list: [] });
export const appState: { state: State | undefined } = $state({
  state: undefined,
});
