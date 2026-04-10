export type Job = {
  id: number;
  name: string;
};

export type PunchType = {
  id: number;
  jobId: number;
  start: string;
  end: string | undefined;
  delta: number | undefined;
  tags: string[] | undefined;
  notes: string | undefined;
};

export class Punch {
  id: number;
  jobId: number;
  start: Date;
  end: Date | undefined;
  delta: number | undefined = $state();
  dDelta: {
    hours: number;
    minutes: number;
    seconds: number;
    str: string;
  } = $derived(this.formatDelta());
  tags: string[] | undefined;
  notes: string | undefined;
  intervalId: number | undefined;

  constructor(p: PunchType) {
    this.id = p.id;
    this.jobId = p.jobId;
    this.start = new Date(p.start);
    this.end = p.end ? new Date(p.end) : undefined;
    this.delta = p.delta;
    this.tags = p.tags;
    this.notes = p.notes;

    if (this.end == undefined) {
      this.intervalId = setInterval(() => {
        this.delta = Date.now() - this.start.getTime();
        console.log(`Delta Time: ${this.delta}`);
      }, 1000);
    }
  }

  clearTimer() {
    clearInterval(this.intervalId);
  }

  formatDelta() {
    const ms = this.delta ?? 0;
    // 1. Calculate total units
    const totalSeconds = Math.floor(ms / 1000);
    const totalMinutes = Math.floor(totalSeconds / 60);

    // 2. Extract remaining parts using the modulo operator (%)
    const seconds = totalSeconds % 60;
    const minutes = totalMinutes % 60;
    const hours = Math.floor(totalMinutes / 60);

    // 3. Helper to add leading zeros
    const pad = (num: number) => num.toString().padStart(2, "0");
    return {
      hours,
      minutes,
      seconds,
      str: `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`,
    };
  }
}

export type State = {
  job: Job;
  theme: string;
};
