use crate::{app_error::AppError, parse_utc};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use turso::{Row, Rows};

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: u64,
    pub name: String,
}

impl Job {
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
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub delta: u64,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
}

impl Punch {
    pub async fn from_rows(mut rows: Rows) -> Result<Vec<Punch>, AppError> {
        let mut punches: Vec<Punch> = Vec::new();
        while let Some(row) = rows.next().await? {
            punches.push(Punch::from_row(&row)?);
        }
        Ok(punches)
    }

    pub fn from_row(row: &Row) -> Result<Punch, AppError> {
        let tags: Option<Vec<String>> = row
            .get::<Option<String>>(5)?
            .map(|s| s.split(',').map(|s| s.to_string()).collect());
        Ok(Punch {
            id: row.get(0)?,
            job_id: row.get(1)?,
            start: parse_utc(row.get(2)?)?.unwrap(),
            end: parse_utc(row.get(3)?)?,
            delta: row.get(4)?,
            tags: tags,
            notes: row.get(6)?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
}

impl Tag {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct AppState {
    pub job: Job,
    pub clocked_in: bool,
    pub theme: String,
}

impl AppState {
    pub fn sql_get() -> &'static str {
        return r#"
        SELECT s.clocked_in, s.theme, j.id, j.name, p.id, p.end
        FROM state s
        JOIN jobs j ON state.job_id = jobs.id
        JOIN punches p ON state.job_id = punches.job_id
        WHERE p.end IS NULL
        LIMIT 1"#;
    }

    pub async fn from_rows(mut rows: Rows) -> Result<AppState, AppError> {
        if let Some(row) = rows.next().await? {
            Ok(AppState {
                job: Job::from_row(&row, 2)?,
                clocked_in: row.get::<u32>(0)? == 1,
                theme: row.get(1)?,
            })
        } else {
            Err("No AppState found")?
        }
    }
}

// pub async fn query_state() -> Result<AppState, AppError> {
//     let job_id: u64;
//     let theme: String;
//     {
//         let conn = get_db().await?;
//         let row = (conn.query("SELECT job_id, theme FROM state", ()).await?)
//             .next()
//             .await?
//             .unwrap();
//         job_id = row.get(0)?;
//         theme = row.get(1)?;
//     }
//     let job = (query_jobs("WHERE id = ?1 LIMIT 1", [job_id]).await?)
//         .pop()
//         .unwrap();
//     Ok(AppState { job, theme })
// }
