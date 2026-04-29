import {
  Job,
  type JobType,
  type PunchType,
  State,
  Punch,
  Tag,
  type TagType,
} from "./types";
import { appState, jobs, punches, tags } from "./state.svelte";
import { invoke } from "@tauri-apps/api/core";

function tryRun(fn: () => Promise<any>) {
  fn().catch((e) => console.log(e));
}

export async function init() {
  tryRun(async () => {
    await getState();
    await getJobs();
    await getTags();
    await changeJob(2);
  });
}

export async function getState() {
  tryRun(async () => {
    let state: State = State.fromType(await invoke("get_state"));
    appState.state = state;
  });
}

export async function changeJob(jobId: number) {
  tryRun(async () => {
    let state: State = await invoke("change_job", { jobId });
    appState.state = state;
    await getPunches(state.job.id);
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

export async function clockIn() {
  tryRun(async () => {
    let pt: PunchType = await invoke("clock_in", {
      jobId: appState.state?.job.id,
      tags: [],
    });
    punches.list.push(Punch.fromType(pt));
    await getState();
  });
}

export async function clockOut() {
  tryRun(async () => {
    let pt: PunchType = await invoke("clock_out", {
      jobId: appState.state?.job.id,
    });
    let punch = Punch.fromType(pt);
    let index = punches.list.findLastIndex((p) => p.id === punch.id);
    punches.list[index] = punch;
    await getState();
  });
}

export async function addPunch(punch: Punch) {
  tryRun(async () => {
    let id: number = await invoke("add_punch", { punch });
    punch.id = id;
    punches.list.push(punch);
    await getState();
  });
}

export async function updatePunch(punch: Punch, listIndex: number) {
  tryRun(async () => {
    await invoke("update_punch", { punch });
    punches.list[listIndex] = punch;
    await getState();
  });
}

export async function getTags() {
  tryRun(async () => {
    let tagTypes: TagType[] = await invoke("get_tags");
    tags.map.clear();
    for (const tag of tagTypes) {
      tags.map.set(tag.id, Tag.fromType(tag));
    }
  });
}
