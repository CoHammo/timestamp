use crate::app_error::AppError;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, MutexGuard};
use turso::{Connection, Row, Rows};

pub struct Db {
    pub conn: Mutex<Connection>,
}

impl Db {
    pub async fn conn(&self) -> MutexGuard<'_, Connection> {
        self.conn.lock().await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppState {
    pub job: Job,
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

    pub async fn get(db: &Db) -> Result<AppState, AppError> {
        let conn = db.conn().await;
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

    pub async fn change_job(db: &Db, job_id: u64) -> Result<AppState, AppError> {
        db.conn()
            .await
            .execute("UPDATE state SET job_id = ?1", [job_id])
            .await?;
        let state = Self::get(db).await?;
        Ok(state)
    }

    pub async fn from_rows(mut rows: Rows) -> Result<AppState, AppError> {
        if let Some(row) = rows.next().await? {
            Ok(AppState {
                job: Job::from_row(&row, 0)?,
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
    pub clocked_in: bool,
}

impl Job {
    pub const TABLE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS jobs (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    );
    INSERT OR IGNORE INTO jobs (name) VALUES ('Job');
    "#;

    pub async fn get_all(db: &Db) -> Result<Vec<Job>, AppError> {
        let conn = db.conn().await;
        let jobs = Job::from_rows(
            conn.query(
                r#"
            SELECT
                j.id,
                j.name,
                CASE WHEN p.id IS NOT NULL AND p.end IS NULL THEN 1 ELSE 0 END AS clocked_in
            FROM jobs j
            LEFT JOIN punches p ON j.id = p.job_id AND p.end IS NULL;
            "#,
                (),
            )
            .await?,
        )
        .await?;
        Ok(jobs)
    }

    pub async fn add(db: &Db, name: String) -> Result<(), AppError> {
        let conn = db.conn().await;
        conn.execute("INSERT into jobs (name) VALUES (?1)", [name])
            .await?;
        Ok(())
    }

    pub async fn update(db: &Db, job: Job) -> Result<(), AppError> {
        let conn = db.conn().await;
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
            clocked_in: row.get::<u64>(col_start + 2)? == 1,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Punch {
    pub id: u64,
    pub job_id: u64,
    pub start: String,
    pub end: Option<String>,
    pub delta_sec: u64,
    pub tags: Vec<u64>,
    pub notes: String,
}

impl Punch {
    pub const TABLE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS punches (
        id INTEGER PRIMARY KEY,
        job_id INTEGER NOT NULL,
        start TEXT NOT NULL,
        end TEXT,
        delta_sec INTEGER NOT NULL DEFAULT 0,
        tags TEXT NOT NULL DEFAULT '',
        notes TEXT NOT NULL DEFAULT '',
        FOREIGN KEY(job_id) REFERENCES jobs(id)
    );
    "#;

    fn tags_string(mut tags: Vec<u64>) -> String {
        if tags.is_empty() {
            return "".to_string();
        } else if tags.len() == 1 {
            return tags.pop().unwrap().to_string();
        } else {
            let mut s = String::new();
            let mut iter = tags.iter();
            s.push_str(&iter.next().unwrap().to_string());
            for tag in iter {
                s.push(',');
                s.push_str(&tag.to_string());
            }
            return s;
        }
    }

    // pub fn new() -> Self {
    //     Self {
    //         id: 0,
    //         job_id: 0,
    //         start: String::new(),
    //         end: None,
    //         delta_sec: 0,
    //         tags: None,
    //         notes: None,
    //     }
    // }

    pub async fn get_for_current_job(db: &Db) -> Result<Vec<Punch>, AppError> {
        let conn = db.conn().await;
        let punches = Punch::from_rows(
            conn.query(
                "SELECT * FROM punches WHERE job_id = (SELECT job_id FROM state) ORDER BY start ASC",
                (),
            )
            .await?,
        )
        .await?;
        Ok(punches)
    }

    pub async fn add(db: &Db, punch: Punch) -> Result<u64, AppError> {
        let conn = db.conn().await;
        if punch.end.is_none() {
            return Err(AppError(
                "End time is required when adding a Punch".to_string(),
            ));
        }
        let mut rows = conn
            .query(
                r#"
            INSERT INTO punches (job_id, start, end, delta_sec, tags, notes)
            VALUES (?1,
                strftime('%Y-%m-%dT%H:%M:%SZ', ?2),
                strftime('%Y-%m-%dT%H:%M:%SZ', ?3),
                unixepoch(?3) - unixepoch(?2),
                ?4,
                ?5)
            RETURNING id
            "#,
                (
                    punch.job_id,
                    punch.start,
                    punch.end,
                    Punch::tags_string(punch.tags),
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

    pub async fn update(db: &Db, punch: Punch) -> Result<(), AppError> {
        let conn = db.conn().await;
        let tags = Punch::tags_string(punch.tags);
        conn.execute(
            format!(
                r#"
            UPDATE punches
            SET job_id = ?1,
                start = strftime('%Y-%m-%dT%H:%M:%SZ', ?2),
                end = strftime('%Y-%m-%dT%H:%M:%SZ', ?3),
                delta_sec = unixepoch(?3) - unixepoch(?2),
                delta_sec = {},
                tags = ?4,
                notes = ?5
            WHERE id = ?6
            "#,
                match punch.end {
                    Some(_) => "unixepoch(?3) - unixepoch(?2)",
                    None => "0",
                }
            ),
            (
                punch.job_id,
                punch.start,
                punch.end,
                tags,
                punch.notes,
                punch.id,
            ),
        )
        .await?;
        Ok(())
    }

    pub async fn delete(db: &Db, punch_id: u64) -> Result<(), AppError> {
        let conn = db.conn().await;
        conn.execute("DELETE FROM punches WHERE id = ?1", [punch_id])
            .await?;
        Ok(())
    }

    pub async fn clock_in(db: &Db, tags: Vec<u64>) -> Result<Punch, AppError> {
        let conn = db.conn().await;
        let mut rows = conn
            .query(
                r#"
                INSERT INTO punches (job_id, start, tags)
                SELECT s.job_id, strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), ?1
                FROM state s
                WHERE NOT EXISTS (
                    SELECT 1 FROM punches
                    WHERE job_id = s.job_id AND end IS NULL LIMIT 1
                )
                RETURNING *;
                "#,
                [Punch::tags_string(tags)],
            )
            .await?;
        if let Some(row) = rows.next().await? {
            Ok(Punch::from_row(&row)?)
        } else {
            Err("Already clocked in")?
        }
    }

    pub async fn re_clock_in(db: &Db) -> Result<(), AppError> {
        let conn = db.conn().await;
        conn.execute(
            r#"
            UPDATE punches SET end = NULL
            WHERE job_id = (SELECT job_id FROM state) AND id = (
                SELECT id FROM punches
                WHERE job_id = (SELECT job_id FROM state)
                ORDER BY start DESC LIMIT 1
            );
            "#,
            (),
        )
        .await?;
        Ok(())
    }

    pub async fn clock_out(db: &Db) -> Result<Punch, AppError> {
        let conn = db.conn().await;
        let mut rows = conn
            .query(
                r#"
            UPDATE punches
            SET end = strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
                delta_sec = unixepoch('now') - unixepoch(punches.start)
            WHERE job_id = (SELECT job_id FROM state) AND end IS NULL
            RETURNING *
            LIMIT 1;"#,
                (),
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
        let tag_str = row.get::<String>(5)?;
        let tags: Vec<u64> = if tag_str.is_empty() {
            Vec::new()
        } else {
            tag_str
                .split(",")
                .map(|s| s.parse::<u64>().unwrap_or(0))
                .collect()
        };
        Ok(Punch {
            id: row.get(0)?,
            job_id: row.get(1)?,
            start: row.get(2)?,
            end: row.get(3)?,
            delta_sec: row.get(4)?,
            tags,
            notes: row.get(6)?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: u64,
    pub name: String,
}

impl Tag {
    pub const TABLE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY,
        name TEXT UNIQUE NOT NULL
    );
    INSERT OR IGNORE INTO tags (name) VALUES ('Office');
    "#;

    pub async fn get_all(db: &Db) -> Result<Vec<Tag>, AppError> {
        let conn = db.conn().await;
        let tags = Self::from_rows(conn.query("SELECT * FROM tags;", ()).await?).await?;
        Ok(tags)
    }

    pub async fn add(db: &Db, name: String) -> Result<(), AppError> {
        let conn = db.conn().await;
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
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }
}
