export class Job {
  id: number = 0;
  name: string = "Job";
}

export class Punch {
  id: number = 0;
  jobId: number;
  label: string = "Work";
  start: Date = new Date();
  end: Date = new Date();
  delta: number | undefined;
  tags: string[] | undefined;

  constructor(jobId: number) {
    this.jobId = jobId;
  }
}
