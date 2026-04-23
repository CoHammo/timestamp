use crate::app_error::AppError;
// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use turso::{Connection, Row, Rows};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppState {
    pub job: Job,
    pub clocked_in: bool,
    pub theme: String,
}

impl AppState {
    pub const TABLE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS state (
        id INTEGER PRIMARY KEY,
        job_id INTEGER NOT NULL DEFAULT 1,
        theme TEXT NOT NULL DEFAULT 'light',
        FOREIGN KEY(job_id) REFERENCES jobs(id) ON DELETE SET DEFAULT
    );
    INSERT OR IGNORE INTO state (id, job_id, theme) VALUES (1, 1, 'light');
    "#;

    pub async fn get(conn: &Connection) -> Result<AppState, AppError> {
        let rows = conn
            .query(
                r#"
                SELECT
                    j.id,
                    j.name,
                    CASE WHEN p.id IS NOT NULL AND p.end IS NULL THEN 1 ELSE 0 END AS clocked_in,
                    s.theme
                FROM state s
                JOIN jobs j ON s.job_id = j.id
                LEFT JOIN punches p ON s.job_id = p.job_id AND p.end IS NULL
                LIMIT 1;
            "#,
                (),
            )
            .await?;
        let state = Self::from_rows(rows).await?;
        Ok(state)
    }

    pub async fn update(conn: &Connection, state: AppState) -> Result<(), AppError> {
        conn.execute(
            "UPDATE state SET job_id = ?1, theme = ?2",
            (state.job.id, state.theme),
        )
        .await?;
        Ok(())
    }

    pub async fn from_rows(mut rows: Rows) -> Result<AppState, AppError> {
        if let Some(row) = rows.next().await? {
            Ok(AppState {
                job: Job::from_row(&row, 0)?,
                clocked_in: row.get::<u64>(2)? == 1,
                theme: row.get(3)?,
            })
        } else {
            Err("No AppState found")?
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: u64,
    pub name: String,
}

impl Job {
    pub const TABLE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS jobs (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    );
    INSERT OR IGNORE INTO jobs (name) VALUES ('Job');
    "#;

    pub async fn get_all(conn: &Connection) -> Result<Vec<Job>, AppError> {
        let jobs = Job::from_rows(conn.query("SELECT * FROM jobs;", ()).await?).await?;
        Ok(jobs)
    }

    pub async fn add(conn: &Connection, name: String) -> Result<(), AppError> {
        conn.execute("INSERT into jobs (name) VALUES (?1)", [name])
            .await?;
        Ok(())
    }

    pub async fn update(conn: &Connection, job: Job) -> Result<(), AppError> {
        conn.execute(
            "UPDATE jobs SET name = ?1 WHERE id = ?2",
            (job.name, job.id),
        )
        .await?;
        Ok(())
    }

    pub async fn from_rows(mut rows: Rows) -> Result<Vec<Job>, AppError> {
        let mut jobs: Vec<Job> = Vec::new();
        while let Some(row) = rows.next().await? {
            jobs.push(Job::from_row(&row, 0)?);
        }
        Ok(jobs)
    }

    pub fn from_row(row: &Row, col_start: usize) -> Result<Job, AppError> {
        Ok(Job {
            id: row.get(col_start)?,
            name: row.get(col_start + 1)?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Punch {
    pub id: u64,
    pub job_id: u64,
    pub start: String,
    pub end: Option<String>,
    pub delta_sec: i64,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
}

impl Punch {
    pub const TABLE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS punches (
        id INTEGER PRIMARY KEY,
        job_id INTEGER NOT NULL,
        start TEXT NOT NULL,
        end TEXT,
        delta_sec INTEGER NOT NULL DEFAULT 0,
        tags TEXT,
        notes TEXT,
        FOREIGN KEY(job_id) REFERENCES jobs(id)
    );
    "#;

    pub fn new() -> Self {
        Self {
            id: 0,
            job_id: 0,
            start: String::new(),
            end: None,
            delta_sec: 0,
            tags: None,
            notes: None,
        }
    }

    pub async fn get_for_job(conn: &Connection, job_id: u64) -> Result<Vec<Punch>, AppError> {
        let punches = Punch::from_rows(
            conn.query(
                "SELECT * FROM punches WHERE job_id = ?1 ORDER BY start ASC",
                [job_id],
            )
            .await?,
        )
        .await?;
        Ok(punches)
    }

    pub async fn add(conn: &Connection, punch: Punch) -> Result<u64, AppError> {
        let mut rows = conn
            .query(
                r#"
            INSERT INTO punches (job_id, start, end, delta_sec, tags, notes)
            VALUES (?1, strftime('%Y-%m-%dT%H:%M:%SZ', ?2), strftime('%Y-%m-%dT%H:%M:%SZ', ?3), unixepoch(?3) - unixepoch(?2), ?4, ?5)
            RETURNING id
            "#,
                (
                    punch.job_id,
                    punch.start,
                    punch.end,
                    punch.tags.map(|s| s.join(",")),
                    punch.notes,
                ),
            )
            .await?;
        let mut id: u64 = 0;
        if let Some(row) = rows.next().await? {
            id = row.get(0)?;
        }
        Ok(id)
    }

    pub async fn update(conn: &Connection, punch: Punch) -> Result<(), AppError> {
        conn.execute(
            r#"
            UPDATE punches
            SET job_id = ?1,
                start = strftime('%Y-%m-%dT%H:%M:%SZ', ?2),
                end = strftime('%Y-%m-%dT%H:%M:%SZ', ?3),
                delta_sec = unixepoch(?3) - unixepoch(?2),
                tags = ?4,
                notes = ?5
            WHERE id = ?6
            "#,
            (
                punch.job_id,
                punch.start,
                punch.end,
                punch.tags.map(|s| s.join(",")),
                punch.notes,
                punch.id,
            ),
        )
        .await?;
        Ok(())
    }

    pub async fn clock_in(
        conn: &Connection,
        job_id: u64,
        tags: Option<Vec<String>>,
    ) -> Result<Punch, AppError> {
        let mut rows = conn
            .query(
                r#"
                INSERT INTO punches (job_id, start, tags)
                SELECT ?1, strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), ?2
                WHERE NOT EXISTS (SELECT 1 FROM punches)
                OR EXISTS (SELECT 1 FROM punches WHERE job_id = ?1 AND end IS NOT NULL LIMIT 1)
                RETURNING *;
                "#,
                (job_id, tags.map(|s| s.join(","))),
            )
            .await?;
        if let Some(row) = rows.next().await? {
            Ok(Punch::from_row(&row)?)
        } else {
            Err("Already clocked in")?
        }
    }

    pub async fn clock_out(conn: &Connection, job_id: u64) -> Result<Punch, AppError> {
        let mut rows = conn
            .query(
                r#"
            UPDATE punches
            SET end = strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
                delta_sec = unixepoch('now') - unixepoch(punches.start)
            WHERE job_id = ?1 AND end IS NULL
            RETURNING *
            LIMIT 1;"#,
                [job_id],
            )
            .await?;
        if let Some(row) = rows.next().await? {
            Ok(Punch::from_row(&row)?)
        } else {
            Err("Tried to clock out when not clocked in")?
        }
    }

    pub async fn from_rows(mut rows: Rows) -> Result<Vec<Punch>, AppError> {
        let mut punches: Vec<Punch> = Vec::new();
        while let Some(row) = rows.next().await? {
            punches.push(Punch::from_row(&row)?);
        }
        Ok(punches)
    }

    pub fn from_row(row: &Row) -> Result<Punch, AppError> {
        Ok(Punch {
            id: row.get(0)?,
            job_id: row.get(1)?,
            start: row.get(2)?,
            end: row.get(3)?,
            delta_sec: row.get(4)?,
            tags: row
                .get::<Option<String>>(5)?
                .map(|s| s.split(',').map(|s| s.to_string()).collect()),
            notes: row.get(6)?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
}

impl Tag {
    pub const TABLE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS tags (
        name TEXT PRIMARY KEY
    );
    INSERT OR IGNORE INTO tags (name) VALUES ('Office');
    "#;

    pub async fn get_all(conn: &Connection) -> Result<Vec<Tag>, AppError> {
        let tags = Self::from_rows(conn.query("SELECT * FROM tags;", ()).await?).await?;
        Ok(tags)
    }

    pub async fn add(conn: &Connection, name: String) -> Result<(), AppError> {
        conn.execute("INSERT INTO tags (name) VALUES (?1)", [name])
            .await?;
        Ok(())
    }

    pub async fn from_rows(mut rows: Rows) -> Result<Vec<Tag>, AppError> {
        let mut tags: Vec<Tag> = Vec::new();
        while let Some(row) = rows.next().await? {
            tags.push(Tag::from_row(&row)?);
        }
        Ok(tags)
    }

    pub fn from_row(row: &Row) -> Result<Tag, AppError> {
        Ok(Tag { name: row.get(0)? })
    }
}
