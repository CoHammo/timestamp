import { Job, Punch } from "$lib";

export const jobs: { list: Job[] } = $state({ list: [] });
export const punches: { list: Punch[] } = $state({ list: [] });
export const currentJob: { job: Job | undefined } = $state({ job: undefined });
