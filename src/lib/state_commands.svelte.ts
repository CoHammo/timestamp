import { type Job, type PunchType, type State, Punch } from "./types.svelte";
import { invoke } from "@tauri-apps/api/core";

export const jobs: { list: Job[] } = $state({ list: [] });
export const punches: { list: Punch[] } = $state({ list: [] });
export const appState: { state: State | undefined } = $state({
  state: undefined,
});
let isClockedIn = $derived(punches.list[0]?.end == undefined);
export function clockedIn() {
  return isClockedIn;
}

export async function getState() {
  appState.state = await invoke("get_state");
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
    await getPunches(appState.state?.job.id ?? 1);
    await getState();
  } catch (e) {
    console.log(e);
  }
}

export async function toggleClockedIn() {
  try {
    if (isClockedIn) {
      let p: PunchType = await invoke("clock_out", {
        jobId: appState.state?.job.id,
      });
      punches.list[0].clearTimer();
      punches.list[0] = new Punch(p);
    } else {
      let p: PunchType = await invoke("clock_in", {
        jobId: appState.state?.job.id,
      });
      punches.list.splice(0, 0, new Punch(p));
    }
  } catch (e) {
    console.log(e);
  }
}

export async function addOrEditPunch(punch: Punch) {
  try {
    if (punch.id == 0) {
      await invoke("add_punch", { punch });
    } else {
      await invoke("edit_punch", { punch });
    }
  } catch (e) {
    console.log(e);
  }
}
