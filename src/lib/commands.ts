import {
  Job,
  type JobType,
  type PunchType,
  State,
  Punch,
} from "./types.svelte";
import { appState, jobs, punches } from "./state.svelte";
import { invoke } from "@tauri-apps/api/core";

function tryRun(fn: () => Promise<void>) {
  fn().catch((e) => console.log(e));
}

export async function init() {
  tryRun(async () => {
    let start: [State, JobType[], PunchType[]] = await invoke("get_start_data");
    appState.state = State.fromType(start[0]);
    jobs.list = start[1].map((j) => Job.fromType(j));
    punches.list = start[2].map((p) => Punch.fromType(p));
  });
}

export async function getState() {
  tryRun(async () => {
    appState.state = State.fromType(await invoke("get_state"));
  });
}

export async function getJobs() {
  tryRun(async () => {
    let jobTypes: JobType[] = await invoke("get_jobs");
    jobs.list = jobTypes.map((j) => Job.fromType(j));
  });
}

export async function getPunches(jobId: number) {
  tryRun(async () => {
    let punchTypes: PunchType[] = await invoke("get_punches", { jobId });
    punches.list = punchTypes.map((p) => Punch.fromType(p));
  });
}

export async function toggleClockIn() {
  tryRun(async () => {
    if (appState.state?.clocked_in) {
      let pt: PunchType = await invoke("clock_out", {
        jobId: appState.state?.job.id,
      });
      punches.list[0].clearTimer();
      punches.list[0] = Punch.fromType(pt);
    } else {
      let pt: PunchType = await invoke("clock_in", {
        jobId: appState.state?.job.id,
      });
      punches.list.splice(0, 0, Punch.fromType(pt));
    }
  });
}

export async function addPunch(punch: Punch) {
  tryRun(async () => {
    let id: number = await invoke("add_punch", { punch });
    punch.id = id;
    punches.list.splice(0, 0, punch);
  });
}

export async function updatePunch(punch: Punch) {
  tryRun(async () => {
    let pt: PunchType = await invoke("edit_punch", { punch });
    punches.list.splice(
      punches.list.findIndex((p) => p.id === punch.id),
      1,
      Punch.fromType(pt),
    );
  });
}
