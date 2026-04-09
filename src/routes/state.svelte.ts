import { type Job, type Punch, type State } from "$lib";
import { invoke } from "@tauri-apps/api/core";

export const jobs: { list: Job[] } = $state({ list: [] });
export const punches: { list: Punch[] } = $state({ list: [] });
export const state: { state: State | undefined } = $state({ state: undefined });

export async function getState() {
  state.state = await invoke("get_state");
  console.log(JSON.stringify(state, null, 2));
}

export async function getJobs() {
  jobs.list = await invoke("get_jobs");
  console.log(JSON.stringify(jobs.list, null, 2));
}

export function correctPunch(punch: Punch) {
  punch.start = new Date(punch.start);
  punch.end = punch.end ? new Date(punch.end) : undefined;
}

export async function getPunches(jobId: number) {
  punches.list = await invoke("get_punches", { jobId });
  for (const punch of punches.list) {
    correctPunch(punch);
  }
  console.log(JSON.stringify(punches.list, null, 2));
}

export async function init() {
  try {
    await getJobs();
    await getPunches(state.state?.job.id ?? 1);
    await getState();
  } catch (e) {
    console.log(e);
  }
}
