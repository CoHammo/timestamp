export type StateType = {
  job: JobType;
  theme: string;
};

export class State {
  job: Job = new Job();
  theme: string = "no theme";

  static fromType(s: StateType) {
    let state = new State();
    state.job = Job.fromType(s.job);
    state.theme = s.theme;
    return state;
  }
}

export type JobType = {
  id: number;
  name: string;
  clocked_in: boolean;
};

export class Job {
  id: number = -1;
  name: string = "No Job";
  clocked_in: boolean = false;

  static fromType(j: JobType) {
    let job = new Job();
    job.id = j.id;
    job.name = j.name;
    job.clocked_in = j.clocked_in;
    return job;
  }
}

export type Delta = {
  hours: number;
  minutes: number;
  seconds: number;
  str: string;
};

export type PunchType = {
  id: number;
  job_id: number;
  start: string;
  end: string | undefined;
  delta_sec: number;
  tags: number[];
  notes: string;
};

export class Punch {
  id: number = 0;
  job_id: number;
  start: Date;
  end: Date | undefined;
  delta_sec: number = 0;
  tags: number[] = [];
  notes: string = "";

  constructor(job_id: number, date: Date = new Date()) {
    this.job_id = job_id;
    this.start = date;
    this.end = date;
  }

  static fromType(p: PunchType) {
    let punch = new Punch(p.job_id, new Date(p.start));
    punch.id = p.id;
    punch.end = p.end ? new Date(p.end) : undefined;
    punch.delta_sec = p.delta_sec;
    punch.tags = p.tags;
    punch.notes = p.notes;
    return punch;
  }

  clone(): Punch {
    let punch = new Punch(this.job_id, new Date(this.start));
    punch.id = this.id;
    punch.end = this.end != undefined ? new Date(this.end) : undefined;
    punch.delta_sec = this.delta_sec;
    punch.tags = [...this.tags];
    punch.notes = this.notes;
    return punch;
  }

  getDelta(): Delta {
    // Calculate total units
    const totalSeconds = Math.floor(
      ((this.end ?? new Date()).getTime() - this.start.getTime()) / 1000,
    );
    const totalMinutes = Math.floor(totalSeconds / 60);

    // Extract remaining parts using the modulo operator (%)
    const seconds = totalSeconds % 60;
    const minutes = totalMinutes % 60;
    const hours = Math.floor(totalMinutes / 60);

    // Helper to add leading zeros
    const pad = (num: number) => num.toString().padStart(2, "0");
    let delta = {
      hours,
      minutes,
      seconds,
      str: `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`,
    };
    return delta;
  }
}

export type TagType = {
  id: number;
  name: string;
};

export class Tag {
  id: number;
  name: string;

  constructor() {
    this.id = 0;
    this.name = "Tag";
  }

  static fromType(t: TagType) {
    let tag = new Tag();
    tag.id = t.id;
    tag.name = t.name;
    return tag;
  }
}
