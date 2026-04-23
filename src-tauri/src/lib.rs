mod app_error;
mod tests;
mod types;

use app_error::AppError;
// use chrono::{DateTime, TimeDelta, Utc};
use tauri::async_runtime::block_on;
use tauri::{Manager, State};
use turso::{Builder, Connection};
use types::*;

// pub fn parse_utc(s: Option<String>) -> Result<Option<DateTime<Utc>>, AppError> {
//     if let Some(s) = s {
//         let dt: DateTime<Utc> = s.parse()?;
//         Ok(Some(dt))
//     } else {
//         Ok(None)
//     }
// }

async fn init_db() -> Result<Connection, AppError> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    let sql = vec![Job::TABLE, Punch::TABLE, Tag::TABLE, AppState::TABLE].concat();
    conn.execute_batch(sql).await?;
    Ok(conn)
}

async fn seed_db(conn: &Connection) -> Result<(), AppError> {
    Job::add(conn, String::from("Christian Challenge")).await?;
    Tag::add(conn, String::from("Discipleship")).await?;
    Punch::add(
        conn,
        Punch {
            id: 0,
            job_id: 1,
            start: String::from("2026-04-20T08:00:00.000Z"),
            end: Some(String::from("2026-04-20T12:00:00.000Z")),
            delta_sec: 0,
            tags: Some(vec![String::from("Discipleship")]),
            notes: Some(String::from("Met with John")),
        },
    )
    .await?;
    Punch::clock_in(conn, 1, None).await?;
    Ok(())
}

#[tauri::command]
async fn get_start_data(
    conn: State<'_, Connection>,
) -> Result<(AppState, Vec<Job>, Vec<Punch>), AppError> {
    let state = AppState::get(&conn).await?;
    let jobs = Job::get_all(&conn).await?;
    let punches = Punch::get_for_job(&conn, state.job.id).await?;
    Ok((state, jobs, punches))
}

#[tauri::command]
async fn get_state(conn: State<'_, Connection>) -> Result<AppState, AppError> {
    AppState::get(&conn).await
}

#[tauri::command]
async fn update_state(conn: State<'_, Connection>, app_state: AppState) -> Result<(), AppError> {
    AppState::update(&conn, app_state).await
}

#[tauri::command]
async fn get_jobs(conn: State<'_, Connection>) -> Result<Vec<Job>, AppError> {
    Job::get_all(&conn).await
}

#[tauri::command]
async fn add_job(conn: State<'_, Connection>, name: String) -> Result<(), AppError> {
    Job::add(&conn, name).await
}

#[tauri::command]
async fn edit_job(conn: State<'_, Connection>, job: Job) -> Result<(), AppError> {
    Job::update(&conn, job).await
}

#[tauri::command]
async fn get_punches(conn: State<'_, Connection>, job_id: u64) -> Result<Vec<Punch>, AppError> {
    Punch::get_for_job(&conn, job_id).await
}

#[tauri::command]
async fn add_punch(conn: State<'_, Connection>, punch: Punch) -> Result<u64, AppError> {
    Punch::add(&conn, punch).await
}

#[tauri::command]
async fn update_punch(conn: State<'_, Connection>, punch: Punch) -> Result<(), AppError> {
    Punch::update(&conn, punch).await
}

#[tauri::command]
async fn clock_in(
    conn: State<'_, Connection>,
    job_id: u64,
    tags: Option<Vec<String>>,
) -> Result<Punch, AppError> {
    Punch::clock_in(&conn, job_id, tags).await
}

#[tauri::command]
async fn clock_out(conn: State<'_, Connection>, job_id: u64) -> Result<Punch, AppError> {
    Punch::clock_out(&conn, job_id).await
}

#[tauri::command]
async fn get_tags(conn: State<'_, Connection>) -> Result<Vec<Tag>, AppError> {
    Tag::get_all(&conn).await
}

#[tauri::command]
async fn add_tag(conn: State<'_, Connection>, name: String) -> Result<(), AppError> {
    Tag::add(&conn, name).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if let Err(e) = block_on(async {
                let conn = init_db().await?;
                seed_db(&conn).await?;
                println!("Database Initialized");
                app.manage(conn);
                Ok::<(), AppError>(())
            }) {
                eprintln!("{}", e.0);
            }
            Ok(())
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_start_data,
            get_state,
            update_state,
            get_jobs,
            add_job,
            edit_job,
            get_punches,
            add_punch,
            update_punch,
            clock_in,
            clock_out,
            get_tags,
            add_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running timestamp app");
}
