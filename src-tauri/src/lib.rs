mod functions;
use dirs::data_dir;
use sqlx::{Error, SqlitePool};
use std::env;
use std::sync::Arc;
use tauri::State;

use functions::sql::Database;
use std::path::PathBuf;
use tokio::runtime::Runtime; // Import tokio runtime

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Hello, {}!", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Create a Tokio runtime to execute async functions
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        let path: PathBuf = data_dir()
            .unwrap_or_else(|| ".".into())
            .join("my_database.db");
        let mut db_url = format!("sqlite://{}", path.display());
        db_url = "sqlite://my_database.db".to_string();
        println!("Database URL: {}", db_url);
        let db = Database::new(&db_url).await; // Await the async new function
        db.get_pool(); // Call the synchronous get_pool function
        db.create_zone_table("test").await.unwrap(); // Await the async create_zone_table function
                                                     //db.listen_for_changes().await; // Await the async listen_for_changes function
    });
    match env::var("APPDATA") {
        Ok(appdata) => println!("APPDATA is: {}", appdata),
        Err(e) => println!("Couldn't read APPDATA: {}", e),
    }
    // Tauri application initialization
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
