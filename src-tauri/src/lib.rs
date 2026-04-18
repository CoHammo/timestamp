mod app_error;
mod seed_sql;
mod types;

use app_error::AppError;
use chrono::{DateTime, TimeDelta, Utc};
use seed_sql::SEED_BATCH;
use std::sync::OnceLock;
use tauri::async_runtime::block_on;
use tokio::sync::Mutex;
use turso::{Builder, Connection, IntoParams, Rows};
use types::*;

pub static DB: OnceLock<Mutex<Connection>> = OnceLock::new();

pub fn parse_utc(s: Option<String>) -> Result<Option<DateTime<Utc>>, AppError> {
    if let Some(s) = s {
        let dt: DateTime<Utc> = s.parse()?;
        Ok(Some(dt))
    } else {
        Ok(None)
    }
}

async fn init_db() -> Result<(), AppError> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(SEED_BATCH).await?;
    DB.set(Mutex::new(conn))
        .or(Err("Database mutex already initialized"))?;
    add_job("Christian Challenge".to_string()).await?;
    add_tag("Discipleship".to_string()).await?;
    add_punch(Punch {
        id: 0,
        job_id: 1,
        start: (Utc::now() - TimeDelta::milliseconds(3200048)),
        end: Some(Utc::now()),
        delta: 0,
        tags: Some(vec!["Discipleship".to_string()]),
        notes: Some("Met with John".to_string()),
    })
    .await?;
    clock_in(1, None).await?;
    Ok(())
}

async fn execute_db(statement: &str, params: impl IntoParams) -> Result<u64, AppError> {
    if let Some(c) = DB.get() {
        let conn = c.lock().await;
        let rows_affected = conn.execute(statement, params).await?;
        Ok(rows_affected)
    } else {
        Err("Database not initialized")?
    }
}

async fn query_db(statement: &str, params: impl IntoParams) -> Result<Rows, AppError> {
    if let Some(c) = DB.get() {
        let conn = c.lock().await;
        let rows = conn.query(statement, params).await?;
        Ok(rows)
    } else {
        Err("Database not initialized")?
    }
}

// async fn seed_db() -> Result<(), AppError> {
//     {
//         let conn = get_db().await?;
//         conn.execute_batch(SEED_BATCH).await?;
//     }
//     add_job("Christian Challenge".to_string()).await?;
//     add_tag("Discipleship".to_string()).await?;
//     add_punch(Punch {
//         id: 0,
//         job_id: 1,
//         start: (Utc::now() - TimeDelta::milliseconds(3200048)),
//         end: Some(Utc::now()),
//         delta: 0,
//         tags: Some(vec!["Discipleship".to_string()]),
//         notes: Some("Met with John".to_string()),
//     })
//     .await?;
//     clock_in(1, None).await?;
//     Ok(())
// }

#[tauri::command]
async fn get_state() -> Result<AppState, AppError> {
    let state = AppState::from_rows(query_db(AppState::sql_get(), ()).await?).await?;
    Ok(state)
}

#[tauri::command]
async fn update_state(state: AppState) -> Result<(), AppError> {
    execute_db(
        "UPDATE state SET job_id = ?1, theme = ?2",
        (state.job.id, state.theme),
    )
    .await?;
    Ok(())
}

#[tauri::command]
async fn get_jobs() -> Result<Vec<Job>, AppError> {
    let jobs = Job::from_rows(query_db("SELECT * FROM jobs", ()).await?).await?;
    Ok(jobs)
}

#[tauri::command]
async fn add_job(name: String) -> Result<(), AppError> {
    execute_db("INSERT INTO jobs (name) VALUES (?1)", [name]).await?;
    Ok(())
}

#[tauri::command]
async fn edit_job(job: Job) -> Result<(), AppError> {
    execute_db(
        "UPDATE jobs SET name = ?1 WHERE id = ?2",
        (job.name, job.id),
    )
    .await?;
    Ok(())
}

#[tauri::command]
async fn get_punches(job_id: u64) -> Result<Vec<Punch>, AppError> {
    let punches = Punch::from_rows(
        query_db(
            "SELECT * FROM punches WHERE job_id = ?1 ORDER BY start DESC",
            [job_id],
        )
        .await?,
    )
    .await?;
    Ok(punches)
}

#[tauri::command]
async fn add_punch(punch: Punch) -> Result<u64, AppError> {
    let delta: u64 = (punch.end.unwrap() - punch.start)
        .num_milliseconds()
        .try_into()?;
    let mut rows = query_db(
        r#"
        INSERT INTO punches (job_id, start, end, delta, tags, notes)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING id
        "#,
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
    let mut id: u64 = 0;
    if let Some(row) = rows.next().await? {
        id = row.get(0)?;
    }
    Ok(id)
}

#[tauri::command]
async fn edit_punch(punch: Punch) -> Result<(), AppError> {
    let delta: u64 = (punch.end.unwrap() - punch.start)
        .num_milliseconds()
        .try_into()?;
    execute_db(
        r#"
        UPDATE punches
        SET start = ?2,
            end = ?3,
            delta = ?4,
            tags = ?5,
            notes = ?6
        WHERE id = ?1
        "#,
        (
            punch.id,
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
async fn clock_in(job_id: u64, tags: Option<Vec<String>>) -> Result<Punch, AppError> {
    let start = Utc::now();
    let rows = query_db(
        r#"
            INSERT INTO punches (job_id, start, tags) VALUES (?1, ?2, ?3)
            RETURNING *;
            "#,
        (job_id, start.to_string(), tags.map(|s| s.join(","))),
    )
    .await?;
    let punch = Punch::from_rows(rows).await?.pop().unwrap();
    Ok(punch)
}

#[tauri::command]
async fn clock_out(job_id: u64) -> Result<Punch, AppError> {
    let end = Utc::now();
    let rows = query_db(
        "SELECT * FROM punches WHERE job_id = ?1 AND end IS NULL LIMIT 1",
        [job_id],
    )
    .await?;
    let punch_opt = Punch::from_rows(rows).await?.pop();
    if let Some(mut punch) = punch_opt {
        let delta: u64 = (end - punch.start).num_milliseconds().try_into()?;
        execute_db(
            "UPDATE punches SET end = ?2, delta = ?3 WHERE id = ?1",
            (punch.id, end.to_string(), delta),
        )
        .await?;
        punch.end = Some(end);
        punch.delta = delta;
        Ok(punch)
    } else {
        Err("Not clocked in")?
    }
}

#[tauri::command]
async fn get_tags() -> Result<Vec<Tag>, AppError> {
    let tags = Tag::from_rows(query_db("SELECT * FROM tags", ()).await?).await?;
    Ok(tags)
}

#[tauri::command]
async fn add_tag(name: String) -> Result<(), AppError> {
    execute_db("INSERT INTO tags (name) VALUES (?1)", [name]).await?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_| {
            if let Err(e) = block_on(async {
                init_db().await?;
                Ok::<(), AppError>(())
            }) {
                eprintln!("{}", e.0);
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
