// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod functions;
use sqlx::{SqlitePool, Error};
use tauri::State;
use std::sync::Arc;
use dirs::data_dir;
use std::path::PathBuf;
use functions::sql::Database;

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Hello, {}!", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, functions::ai::utils::call_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let path: PathBuf = data_dir().unwrap_or_else(|| ".".into())
    .join("my_database.db");

    let db_url = format!("sqlite://{}", path.display());
    Database::new(&db_url);

}
