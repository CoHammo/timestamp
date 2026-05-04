mod app_error;
mod tests;
mod types;

use app_error::AppError;
use tauri::async_runtime::block_on;
use tauri::{Manager, State};
use tokio::sync::Mutex;
use turso::{Builder, Connection};
use types::*;

async fn init_db() -> Result<Connection, AppError> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    let sql = vec![Job::TABLE, Punch::TABLE, Tag::TABLE, AppState::TABLE].concat();
    conn.execute_batch(sql).await?;
    Ok(conn)
}

async fn seed_db(db: &Db) -> Result<(), AppError> {
    Job::add(db, "Christian Challenge".to_string()).await?;
    Tag::add(db, "Discipleship".to_string()).await?;
    Punch::add(
        db,
        Punch {
            id: 0,
            job_id: 2,
            start: "2026-04-20T08:00:00.000Z".to_string(),
            end: Some("2026-04-20T12:00:00.000Z".to_string()),
            delta_sec: 0,
            tags: vec![2],
            notes: "Met with John".to_string(),
        },
    )
    .await?;
    AppState::change_job(db, 2).await?;
    Punch::clock_in(db, Vec::new()).await?;
    Ok(())
}

#[tauri::command]
async fn get_state(db: State<'_, Db>) -> Result<AppState, AppError> {
    AppState::get(&db).await
}

#[tauri::command]
async fn change_job(db: State<'_, Db>, job_id: u64) -> Result<AppState, AppError> {
    AppState::change_job(&db, job_id).await
}

#[tauri::command]
async fn get_jobs(db: State<'_, Db>) -> Result<Vec<Job>, AppError> {
    let jobs = Job::get_all(&db).await?;
    Ok(jobs)
}

#[tauri::command]
async fn add_job(db: State<'_, Db>, name: String) -> Result<(), AppError> {
    Job::add(&db, name).await
}

#[tauri::command]
async fn update_job(db: State<'_, Db>, job: Job) -> Result<(), AppError> {
    Job::update(&db, job).await
}

#[tauri::command]
async fn get_punches(db: State<'_, Db>) -> Result<Vec<Punch>, AppError> {
    let punches = Punch::get_for_current_job(&db).await?;
    Ok(punches)
}

#[tauri::command]
async fn add_punch(db: State<'_, Db>, punch: Punch) -> Result<u64, AppError> {
    Punch::add(&db, punch).await
}

#[tauri::command]
async fn update_punch(db: State<'_, Db>, punch: Punch) -> Result<(), AppError> {
    Punch::update(&db, punch).await
}

#[tauri::command]
async fn delete_punch(db: State<'_, Db>, punch_id: u64) -> Result<(), AppError> {
    Punch::delete(&db, punch_id).await
}

#[tauri::command]
async fn clock_in(db: State<'_, Db>, tags: Vec<u64>) -> Result<Punch, AppError> {
    Punch::clock_in(&db, tags).await
}

#[tauri::command]
async fn re_clock_in(db: State<'_, Db>) -> Result<(), AppError> {
    Punch::re_clock_in(&db).await
}

#[tauri::command]
async fn clock_out(db: State<'_, Db>) -> Result<Punch, AppError> {
    Punch::clock_out(&db).await
}

#[tauri::command]
async fn get_tags(db: State<'_, Db>) -> Result<Vec<Tag>, AppError> {
    Tag::get_all(&db).await
}

#[tauri::command]
async fn add_tag(db: State<'_, Db>, name: String) -> Result<(), AppError> {
    Tag::add(&db, name).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if let Err(e) = block_on(async {
                let db = Db {
                    conn: Mutex::new(init_db().await?),
                };
                seed_db(&db).await?;
                println!("Database Initialized");
                app.manage(db);
                Ok::<(), AppError>(())
            }) {
                eprintln!("{}", e.0);
                Err("noooo".into())
            } else {
                Ok(())
            }
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_state,
            change_job,
            get_jobs,
            add_job,
            update_job,
            get_punches,
            add_punch,
            update_punch,
            delete_punch,
            clock_in,
            re_clock_in,
            clock_out,
            get_tags,
            add_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running timestamp app");
}
