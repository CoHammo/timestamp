export type Job = {
  id: number;
  name: string;
};

export type Punch = {
  id: number;
  jobId: number;
  start: Date;
  end: Date | undefined;
  delta: number | undefined;
  tags: string[] | undefined;
  notes: string | undefined;
};

export type State = {
  job: Job;
  theme: string;
};
