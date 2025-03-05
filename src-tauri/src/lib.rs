mod functions;
use dirs::data_dir;
use sqlx::{Error, SqlitePool};
use std::env;
use std::sync::Arc;

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
    /* 
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
    */
    // Tauri application initialization
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init()) // 確保檔案系統可用
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            functions::ai::generate_rules::ai_create_rule,
            functions::ai::summarize::ai_summarize_all_files,
            functions::ai::summarize::ai_summarize_one_file,
            functions::ai::sort::ai_sort,
            functions::ai::renew_rules::ai_renew_rules,
            functions::file::get_summary_of_one_file,
            functions::file::set_summary_of_one_file,
            functions::file::get_move_history,
            functions::file::get_file_tree,
            functions::system::get_api_key,
            functions::system::set_api_key,
            functions::zone::create_zone,
            functions::zone::get_zone_list,
            functions::file::get_summary_of_one_file,
            functions::file::set_summary_of_one_file,
            functions::zone::get_zone_rules,
            functions::zone::set_zone_rules,
            functions::file::move_file,
            functions::file::get_ignore_list,
            functions::file::set_ignore_list,
            functions::system::get_api_key,
            functions::system::set_api_key,
            functions::file::get_move_history,
            functions::zone::delete_zone,
            functions::file::get_project_file,
            functions::file::set_project_file,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
