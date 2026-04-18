pub static SEED_BATCH: &str = r#"
CREATE TABLE IF NOT EXISTS jobs (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO jobs (name) VALUES ('Job');

CREATE TABLE IF NOT EXISTS punches (
    id INTEGER PRIMARY KEY,
    job_id INTEGER NOT NULL,
    start TEXT NOT NULL,
    end TEXT,
    delta INTEGER,
    tags TEXT,
    notes TEXT,
    FOREIGN KEY(job_id) REFERENCES jobs(id)
);

CREATE TABLE IF NOT EXISTS tags (
    name TEXT PRIMARY KEY
);
INSERT INTO tags (name) VALUES ('Admin');

CREATE TABLE IF NOT EXISTS state (
    id INTEGER PRIMARY KEY,
    job_id INTEGER NOT NULL DEFAULT 1,
    clocked_in INTEGER NOT NULL,
    theme TEXT NOT NULL DEFAULT 'light',
    FOREIGN KEY(job_id) REFERENCES jobs(id) ON DELETE SET DEFAULT
);
INSERT INTO state (job_id, clocked_in, theme) VALUES (1, 0, 'light');
"#;
