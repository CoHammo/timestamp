mod app_error;
mod types;
use app_error::AppError;
use chrono::{DateTime, Utc};
use std::sync::{Mutex, MutexGuard, OnceLock};
use tauri::async_runtime::block_on;
use turso::{Builder, Connection, Database};
use types::{JOB_TABLE, PUNCH_TABLE, TAG_TABLE};

pub struct Db(Database, Connection);

pub static DB: OnceLock<Mutex<Db>> = OnceLock::new();

pub fn get_db<'a>() -> Result<MutexGuard<'a, Db>, AppError> {
    Ok(DB.get().unwrap().lock()?)
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
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running timestamp app");
}
