import { type Job, type PunchType, type State, Punch } from "$lib";
import { invoke } from "@tauri-apps/api/core";

export const jobs: { list: Job[] } = $state({ list: [] });
export const punches: { list: Punch[] } = $state({ list: [] });
export const things: { state: State | undefined } = $state({
  state: undefined,
});

export async function getState() {
  things.state = await invoke("get_state");
}

export async function getJobs() {
  jobs.list = await invoke("get_jobs");
}

export async function getPunches(jobId: number) {
  let ps: PunchType[] = await invoke("get_punches", { jobId });
  punches.list = [];
  for (const punch of ps) {
    punches.list.push(new Punch(punch));
  }
}

export async function init() {
  try {
    await getJobs();
    await getPunches(things.state?.job.id ?? 1);
    await getState();
  } catch (e) {
    console.log(e);
  }
}
