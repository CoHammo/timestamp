export type JobType = {
  id: number;
  name: string;
};

export class Job {
  id: number = -1;
  name: string = "No Job";

  static fromType(j: JobType) {
    let job = new Job();
    job.id = j.id;
    job.name = j.name;
    return job;
  }
}

export type PunchType = {
  id: number;
  job_id: number;
  start: string;
  end: string | undefined;
  delta_sec: number;
  tags: string[] | undefined;
  notes: string | undefined;
};

export class Punch {
  id: number;
  job_id: number;
  start: Date;
  end: Date | undefined;
  delta_sec: number = 0;
  get delta() {
    return this.delta_sec;
  }
  set delta(value: number) {
    this.delta_sec = value;
    this.formatDelta();
  }
  dDelta: {
    hours: number;
    minutes: number;
    seconds: number;
    str: string;
  } = $state({ hours: 0, minutes: 0, seconds: 0, str: "00:00:00" });
  tags: string[] | undefined;
  notes: string | undefined;
  #intervalId: number | undefined;

  constructor(job_id: number, date: Date = new Date()) {
    this.id = 0;
    this.job_id = job_id;
    this.start = date;
    this.end = date;
    this.delta = 0;
  }

  static fromType(p: PunchType) {
    let punch = new Punch(p.job_id, new Date(p.start));
    punch.id = p.id;
    punch.end = p.end ? new Date(p.end) : undefined;
    punch.delta = p.delta_sec;
    punch.tags = p.tags;
    punch.notes = p.notes;
    punch.setTimer();
    return punch;
  }

  clone(): Punch {
    let punch = new Punch(this.job_id, new Date(this.start));
    punch.id = this.id;
    punch.end = this.end != undefined ? new Date(this.end) : undefined;
    punch.delta = this.delta;
    punch.tags = this.tags != undefined ? [...this.tags] : undefined;
    punch.notes = this.notes;
    punch.setTimer();
    return punch;
  }

  setTimer() {
    if (this.end == undefined && this.#intervalId === undefined) {
      this.#intervalId = setInterval(() => {
        this.delta = Math.floor((Date.now() - this.start.getTime()) / 1000);
      }, 1000);
    }
  }

  clearTimer() {
    clearInterval(this.#intervalId);
    this.#intervalId = undefined;
  }

  formatDelta() {
    // 1. Calculate total units
    // const totalSeconds = Math.floor(this.delta / 1000);
    const totalMinutes = Math.floor(this.delta / 60);

    // 2. Extract remaining parts using the modulo operator (%)
    const seconds = this.delta % 60;
    const minutes = totalMinutes % 60;
    const hours = Math.floor(totalMinutes / 60);

    // 3. Helper to add leading zeros
    const pad = (num: number) => num.toString().padStart(2, "0");
    this.dDelta = {
      hours,
      minutes,
      seconds,
      str: `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`,
    };
    return this.dDelta;
  }
}

export type StateType = {
  job: JobType;
  clocked_in: boolean;
  theme: string;
};

export class State {
  job: Job = new Job();
  clocked_in: boolean = false;
  theme: string = "no theme";

  static fromType(s: StateType) {
    let state = new State();
    state.job = Job.fromType(s.job);
    state.clocked_in = s.clocked_in;
    state.theme = s.theme;
    return state;
  }
}
