mod app_error;
mod types;
use app_error::AppError;
use chrono::{DateTime, Utc};
use std::sync::OnceLock;
use tauri::async_runtime::block_on;
use tokio::sync::{Mutex, MutexGuard};
use turso::{Builder, Connection, Database};
use types::{Job, Punch};
use types::{JOB_TABLE, PUNCH_TABLE, TAG_TABLE};

use crate::types::{query_jobs, query_punches};

pub struct Db(Database, Connection);

pub static DB: OnceLock<Mutex<Db>> = OnceLock::new();

pub async fn get_db<'a>() -> Result<MutexGuard<'a, Db>, AppError> {
    Ok(DB.get().unwrap().lock().await)
}

pub fn parse_utc(s: Option<String>) -> Result<Option<DateTime<Utc>>, AppError> {
    if let Some(s) = s {
        let dt: DateTime<Utc> = s.parse()?;
        Ok(Some(dt))
    } else {
        Ok(None)
    }
}

async fn make_db() -> Result<Db, AppError> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;

    conn.execute(JOB_TABLE, ()).await?;
    conn.execute(PUNCH_TABLE, ()).await?;
    conn.execute(TAG_TABLE, ()).await?;

    Ok(Db(db, conn))
}

#[tauri::command]
async fn get_jobs() -> Result<Vec<Job>, AppError> {
    let jobs = query_jobs("", ()).await?;
    Ok(jobs)
}

#[tauri::command]
async fn add_job(name: String) -> Result<(), AppError> {
    let conn = &get_db().await?.1;
    conn.execute("INSERT INTO jobs (name) VALUES (?1)", [name])
        .await?;
    Ok(())
}

#[tauri::command]
async fn get_punches(job_id: u64) -> Result<Vec<Punch>, AppError> {
    let punches = query_punches("WHERE job_id = ?1", [job_id]).await?;
    Ok(punches)
}

#[tauri::command]
async fn add_punch(
    job_id: u64,
    start: String,
    end: String,
    tags: Option<Vec<String>>,
    notes: Option<String>,
) -> Result<(), AppError> {
    let start_utc: DateTime<Utc> = start.parse()?;
    let end_utc: DateTime<Utc> = end.parse()?;
    let delta = (end_utc - start_utc).num_milliseconds();
    let mut params: Vec<String> = vec![
        job_id.to_string(),
        start_utc.to_string(),
        end_utc.to_string(),
        delta.to_string(),
    ];
    if let Some(tgs) = tags {
        params.push(tgs.join(","));
    }
    if let Some(nts) = notes {
        params.push(nts);
    }

    let conn = &get_db().await?.1;
    conn.execute(
        r#"INSERT INTO punches
            (job_id, start, end, delta, tags, notes)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
        params,
    )
    .await?;
    Ok(())
}

#[tauri::command]
async fn clock_in(job_id: u64, tags: Option<Vec<String>>) -> Result<(), AppError> {
    let start = Utc::now();
    let mut params: Vec<String> = vec![job_id.to_string(), start.to_string()];
    if let Some(tgs) = tags {
        params.push(tgs.join(","));
    }
    let conn = &get_db().await?.1;
    conn.execute(
        r#"INSERT INTO punches
            (job_id, start, tags)
            VALUES (?1, ?2, ?3)"#,
        params,
    )
    .await?;
    Ok(())
}

#[tauri::command]
async fn clock_out(
    job_id: u64,
    tags: Option<Vec<String>>,
    notes: Option<String>,
) -> Result<(), AppError> {
    let end = Utc::now();
    let punches = query_punches("WHERE job_id = ?1 AND end IS NULL LIMIT 1", [job_id]).await?;
    if let Some(punch) = punches.get(0) {
        let delta = (end - punch.start).num_milliseconds();
        let mut params: Vec<String> =
            vec![punch.id.to_string(), end.to_string(), delta.to_string()];
        if let Some(tgs) = tags {
            params.push(tgs.join(","));
        }
        if let Some(nts) = notes {
            params.push(nts);
        }
        let conn = &get_db().await?.1;
        conn.execute(
            r#"
            UPDATE punches
            SET end = ?2,
                delta = ?3,
                tags = ?4,
                notes = ?5
            WHERE id = ?1"#,
            params,
        )
        .await?;
        Ok(())
    } else {
        Err("Not clocked in")?
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_| {
            let _: Result<(), AppError> = block_on(async move {
                match DB.set(Mutex::new(make_db().await?)) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(AppError("Database Mutex Already Initialized".to_string())),
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_jobs,
            add_job,
            get_punches,
            add_punch,
            clock_in,
            clock_out,
        ])
        .run(tauri::generate_context!())
        .expect("error while running timestamp app");
}
