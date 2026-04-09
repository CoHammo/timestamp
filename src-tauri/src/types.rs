use crate::{app_error::AppError, get_db, parse_utc};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use turso::IntoParams;

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: u64,
    pub name: String,
}

pub async fn query_jobs(
    query: &'static str,
    params: impl IntoParams,
) -> Result<Vec<Job>, AppError> {
    let conn = get_db().await?;
    let mut rows = conn
        .query(format!("SELECT * FROM jobs {}", query), params)
        .await?;
    let mut jobs: Vec<Job> = Vec::new();
    while let Some(row) = rows.next().await? {
        jobs.push(Job {
            id: row.get(0)?,
            name: row.get(1)?,
        });
    }
    Ok(jobs)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Punch {
    pub id: u64,
    pub job_id: u64,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub delta: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
}

pub async fn query_punches(
    query: &'static str,
    params: impl IntoParams,
) -> Result<Vec<Punch>, AppError> {
    let conn = get_db().await?;
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

pub async fn query_tags(
    query: &'static str,
    params: impl IntoParams,
) -> Result<Vec<String>, AppError> {
    let conn = get_db().await?;
    let mut rows = conn
        .query(format!("SELECT * FROM tags {}", query), params)
        .await?;
    let mut tags: Vec<String> = Vec::new();
    while let Some(row) = rows.next().await? {
        tags.push(row.get(0)?);
    }
    Ok(tags)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppState {
    pub job: Job,
    pub theme: String,
}

pub async fn query_state() -> Result<AppState, AppError> {
    let job_id: u64;
    let theme: String;
    {
        let conn = get_db().await?;
        let row = (conn.query("SELECT job_id, theme FROM state", ()).await?)
            .next()
            .await?
            .unwrap();
        job_id = row.get(0)?;
        theme = row.get(1)?;
    }
    let job = (query_jobs("WHERE id = ?1 LIMIT 1", [job_id]).await?)
        .pop()
        .unwrap();
    Ok(AppState { job, theme })
}
