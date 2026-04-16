mod app_error;
mod seed_sql;
mod types;

use app_error::AppError;
use chrono::{DateTime, TimeDelta, Utc};
use seed_sql::SEED_BATCH;
use std::sync::OnceLock;
use tauri::async_runtime::block_on;
use tokio::sync::{Mutex, MutexGuard};
use turso::{Builder, Connection};
use types::*;

pub static DB: OnceLock<Mutex<Connection>> = OnceLock::new();

pub async fn get_db<'a>() -> Result<MutexGuard<'a, Connection>, AppError> {
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

async fn connect_db() -> Result<(), AppError> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    DB.set(Mutex::new(conn))
        .or(Err("Database mutex already initialized"))?;
    Ok(())
}

async fn seed_db() -> Result<(), AppError> {
    {
        let conn = get_db().await?;
        conn.execute_batch(SEED_BATCH).await?;
    }
    add_job("Christian Challenge".to_string()).await?;
    add_tag("Discipleship".to_string()).await?;
    add_punch(Punch {
        id: 0,
        job_id: 1,
        start: (Utc::now() - TimeDelta::milliseconds(3200048)),
        end: Some(Utc::now()),
        delta: None,
        tags: Some(vec!["Discipleship".to_string()]),
        notes: Some("Met with John".to_string()),
    })
    .await?;
    clock_in(1, None).await?;
    Ok(())
}

#[tauri::command]
async fn get_state() -> Result<AppState, AppError> {
    let state = query_state().await?;
    Ok(state)
}

#[tauri::command]
async fn update_state(state: AppState) -> Result<(), AppError> {
    let conn = get_db().await?;
    conn.execute(
        "UPDATE state SET job_id = ?1, theme = ?2",
        (state.job.id, state.theme),
    )
    .await?;
    Ok(())
}

#[tauri::command]
async fn get_jobs() -> Result<Vec<Job>, AppError> {
    let jobs = query_jobs("", ()).await?;
    Ok(jobs)
}

#[tauri::command]
async fn add_job(name: String) -> Result<(), AppError> {
    let conn = get_db().await?;
    conn.execute("INSERT INTO jobs (name) VALUES (?1)", [name])
        .await?;
    Ok(())
}

#[tauri::command]
async fn edit_job(job: Job) -> Result<(), AppError> {
    let conn = get_db().await?;
    conn.execute(
        "UPDATE jobs SET name = ?1 WHERE id = ?2",
        (job.name, job.id),
    )
    .await?;
    Ok(())
}

#[tauri::command]
async fn get_punches(job_id: u64) -> Result<Vec<Punch>, AppError> {
    let punches = query_punches("WHERE job_id = ?1 ORDER BY start DESC", [job_id]).await?;
    Ok(punches)
}

#[tauri::command]
async fn add_punch(punch: Punch) -> Result<(), AppError> {
    let delta: u64 = (punch.end.unwrap() - punch.start)
        .num_milliseconds()
        .try_into()?;
    let conn = get_db().await?;
    conn.execute(
        r#"INSERT INTO punches
            (job_id, start, end, delta, tags, notes)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
        (
            punch.job_id,
            punch.start.to_string(),
            punch.end.map(|e| e.to_string()),
            delta,
            punch.tags.map(|s| s.join(",")),
            punch.notes,
        ),
    )
    .await?;
    Ok(())
}

#[tauri::command]
async fn edit_punch(punch: Punch) -> Result<(), AppError> {
    let delta: u64 = (punch.end.unwrap() - punch.start)
        .num_milliseconds()
        .try_into()?;
    let conn = get_db().await?;
    conn.execute(
        r#"INSERT INTO punches
            (job_id, start, end, delta, tags, notes)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ON CONFLICT DO UPDATE
            SET end = ?3, delta = ?4, tags = ?5, notes = ?6
            WHERE job_id = ?1 AND start = ?2"#,
        (
            punch.job_id,
            punch.start.to_string(),
            punch.end.map(|e| e.to_string()),
            delta,
            punch.tags.map(|s| s.join(",")),
            punch.notes,
        ),
    )
    .await?;
    Ok(())
}

// #[tauri::command]
// async fn add_punch(
//     job_id: u64,
//     start: String,
//     end: String,
//     tags: Option<Vec<String>>,
//     notes: Option<String>,
// ) -> Result<(), AppError> {
//     let start_utc: DateTime<Utc> = start.parse()?;
//     let end_utc: DateTime<Utc> = end.parse()?;
//     let delta = (end_utc - start_utc).num_milliseconds();
//     let mut params: Vec<String> = vec![
//         job_id.to_string(),
//         start_utc.to_string(),
//         end_utc.to_string(),
//         delta.to_string(),
//     ];
//     if let Some(tgs) = tags {
//         params.push(tgs.join(","));
//     }
//     if let Some(nts) = notes {
//         params.push(nts);
//     }

//     let conn = get_db().await?;
//     conn.execute(
//         r#"INSERT INTO punches
//             (job_id, start, end, delta, tags, notes)
//             VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
//         params,
//     )
//     .await?;
//     Ok(())
// }

// #[tauri::command]
// async fn edit_punch(
//     id: u64,
//     start: String,
//     end: String,
//     tags: Option<Vec<String>>,
//     notes: Option<String>,
// ) -> Result<(), AppError> {
//     let conn = get_db().await?;
//     let start_utc: DateTime<Utc> = start.parse()?;
//     let end_utc: DateTime<Utc> = end.parse()?;
//     let delta = (end_utc - start_utc).num_milliseconds();
//     let mut params: Vec<String> = vec![
//         id.to_string(),
//         start_utc.to_string(),
//         end_utc.to_string(),
//         delta.to_string(),
//     ];
//     if let Some(tgs) = tags {
//         params.push(tgs.join(","));
//     }
//     if let Some(nts) = notes {
//         params.push(nts);
//     }
//     conn.execute(
//         r#"
//         UPDATE punches SET
//             start = ?2,
//             end = ?3,
//             delta = ?4,
//             tags = ?5,
//             notes = ?6
//         WHERE id = ?1"#,
//         params,
//     )
//     .await?;

//     Ok(())
// }

#[tauri::command]
async fn clock_in(job_id: u64, tags: Option<Vec<String>>) -> Result<Punch, AppError> {
    let start = Utc::now();
    let mut params: Vec<String> = vec![job_id.to_string(), start.to_string()];
    if let Some(tgs) = tags {
        params.push(tgs.join(","));
    }
    {
        let conn = get_db().await?;
        conn.execute(
            "INSERT INTO punches (job_id, start, tags) VALUES (?1, ?2, ?3)",
            params,
        )
        .await?;
    }
    let punch = query_punches("WHERE job_id = ?1 AND end IS NULL LIMIT 1", [job_id])
        .await?
        .pop()
        .unwrap();
    Ok(punch)
}

#[tauri::command]
async fn clock_out(job_id: u64) -> Result<Punch, AppError> {
    let end = Utc::now();
    let punch_opt = query_punches("WHERE job_id = ?1 AND end IS NULL LIMIT 1", [job_id])
        .await?
        .pop();
    if let Some(mut punch) = punch_opt {
        let delta: u64 = (end - punch.start).num_milliseconds().try_into()?;
        let conn = get_db().await?;
        conn.execute(
            "UPDATE punches SET end = ?2, delta = ?3 WHERE id = ?1",
            (punch.id, end.to_string(), delta),
        )
        .await?;
        punch.end = Some(end);
        punch.delta = Some(delta);
        Ok(punch)
    } else {
        Err("Not clocked in")?
    }
}

#[tauri::command]
async fn get_tags() -> Result<Vec<String>, AppError> {
    let tags = query_tags("", ()).await?;
    Ok(tags)
}

#[tauri::command]
async fn add_tag(name: String) -> Result<(), AppError> {
    let conn = get_db().await?;
    conn.execute("INSERT INTO tags (name) VALUES (?1)", [name])
        .await?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_| {
            let res: Result<(), AppError> = block_on(async move {
                connect_db().await?;
                seed_db().await?;
                Ok(())
            });
            match res {
                Ok(_) => {}
                Err(e) => eprintln!("{}", e.0),
            }
            Ok(())
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_state,
            update_state,
            get_jobs,
            add_job,
            edit_job,
            get_punches,
            add_punch,
            edit_punch,
            clock_in,
            clock_out,
            get_tags,
            add_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running timestamp app");
}
