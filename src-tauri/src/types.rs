use crate::{app_error::AppError, get_db, parse_utc};
use chrono::{DateTime, Utc};
use serde::Serialize;
use turso::IntoParams;

#[derive(Serialize)]
pub struct Job {
    pub id: u64,
    pub name: String,
}
pub static JOB_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS jobs (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
)"#;

pub async fn query_jobs(
    query: &'static str,
    params: impl IntoParams,
) -> Result<Vec<Job>, AppError> {
    let conn = &get_db().await?.1;
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
    pub id: u64,
    pub job_id: u64,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub delta: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
}
pub static PUNCH_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS punches (
    id INTEGER PRIMARY KEY,
    job_id INTEGER NOT NULL,
    start TEXT NOT NULL,
    end TEXT,
    delta INTEGER,
    tags TEXT,
    notes TEXT,
    FOREIGN KEY(job_id) REFERENCES jobs(id)
)"#;

pub async fn query_punches(
    query: &'static str,
    params: impl IntoParams,
) -> Result<Vec<Punch>, AppError> {
    let conn = &get_db().await?.1;
    let mut rows = conn
        .query(format!("SELECT * FROM punches {}", query), params)
        .await?;
    let mut punches: Vec<Punch> = Vec::new();
    while let Some(row) = rows.next().await? {
        let tags: Option<Vec<String>> = row
            .get::<Option<String>>(5)?
            .map(|s| s.split(',').map(|s| s.to_string()).collect());
        punches.push(Punch {
            id: row.get(0)?,
            job_id: row.get(1)?,
            start: parse_utc(row.get(2)?)?.unwrap(),
            end: parse_utc(row.get(3)?)?,
            delta: row.get(4)?,
            tags: tags,
            notes: row.get(6)?,
        });
    }
    Ok(punches)
}

pub static TAG_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS tags (
    name TEXT PRIMARY KEY
)
"#;
