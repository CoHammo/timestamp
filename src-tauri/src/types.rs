use crate::{app_error::AppError, get_db, parse_utc};
use chrono::{DateTime, Utc};
use serde::Serialize;
use turso::IntoParams;

#[derive(Serialize)]
pub struct Job {
    pub id: i64,
    pub name: String,
}
pub static JOB_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS jobs (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE
)"#;

pub async fn query_jobs(
    query: &'static str,
    params: impl IntoParams,
) -> Result<Vec<Job>, AppError> {
    let conn = &get_db()?.1;
    let mut rows = conn
        .query(format!("SELECT * FROM jobs {}", query), params)
        .await?;
    let mut punches: Vec<Job> = Vec::new();
    while let Some(row) = rows.next().await? {
        punches.push(Job {
            id: row.get(0)?,
            name: row.get(1)?,
        });
    }
    Ok(punches)
}

#[derive(Serialize)]
pub struct Punch {
    pub id: i64,
    pub job_id: i64,
    pub label: String,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub delta: Option<i64>,
    pub tags: Option<Vec<String>>,
}
pub static PUNCH_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS punches (
    id INTEGER PRIMARY KEY,
    job_id INTEGER NOT NULL,
    label TEXT NOT NULL DEFAULT 'Work',
    start TEXT NOT NULL,
    end TEXT,
    delta INTEGER,
    tags TEXT,
    FOREIGN KEY(job_id) REFERENCES jobs(id)
)"#;

pub async fn query_punches(
    query: &'static str,
    params: impl IntoParams,
) -> Result<Vec<Punch>, AppError> {
    let conn = &get_db()?.1;
    let mut rows = conn
        .query(format!("SELECT * FROM punches {}", query), params)
        .await?;
    let mut punches: Vec<Punch> = Vec::new();
    while let Some(row) = rows.next().await? {
        let tags: Option<Vec<String>> = row
            .get::<Option<String>>(6)?
            .map(|s| s.split(',').map(|s| s.to_string()).collect());
        punches.push(Punch {
            id: row.get(0)?,
            job_id: row.get(1)?,
            label: row.get(2)?,
            start: parse_utc(row.get(3)?)?.unwrap(),
            end: parse_utc(row.get(4)?)?,
            delta: row.get(5)?,
            tags: tags,
        });
    }
    Ok(punches)
}

pub static TAG_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS tags (
    name TEXT PRIMARY KEY
)
"#;
