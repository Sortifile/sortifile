use crate::functions;
use crate::functions::ai::utils;
use crate::functions::file;
use crate::functions::sql;
use crate::functions::system::get_appdata_dir;
use crate::functions::system;
use crate::functions::zone;
use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use glob::Pattern;
use serde_json::to_string;
use serde_json::{json, Value};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tauri::command;
use tauri::path::BaseDirectory;
use tauri::Manager;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

/// For ai_sort and ai_renew_rules we now include the AppHandle parameter
/// so that we can spawn the proper sidecar.

#[tauri::command]
pub async fn ai_summarize_all_files(
    app: tauri::AppHandle,
    zone_name: &str,
    zone_path: &str,
) -> Result<String, String> {
    let db = sql::get_db().await;
    process_directory_recursively(app, zone_path.to_string(), zone_name, "")
        .await
        .unwrap();
    Ok("good".to_string())
}

/// The rest of the functions below are helper functions for processing directories and files.

#[command]
async fn process_directory_recursively(
    app: tauri::AppHandle,
    dir_path: String,
    zone_name: &str,
    path_to_sort: &str,
) -> Result<(), String> {
    //println!("Processing directory: {}", dir_path);
    let base = Path::new(&dir_path);
    if !base.exists() {
        return Err(format!("Path '{}' does not exist.", dir_path));
    }

    // Load ignore patterns from a ".sortifile-ignore" file in the root directory, if present.
    let ignore_file = base.join(".sortifile-ignore");
    let ignore_patterns = if ignore_file.exists() {
        load_ignore_patterns(&ignore_file)
            .map_err(|e| format!("Failed to load ignore patterns: {}", e))?
    } else {
        Vec::new()
    };
    // Process the directory recursively.
    process_path(app, base, base, &ignore_patterns, zone_name, path_to_sort).await;
    Ok(())
}

/// Loads ignore patterns from the provided ".ignore" file.
fn load_ignore_patterns(ignore_file: &Path) -> io::Result<Vec<Pattern>> {
    let content = fs::read_to_string(ignore_file)?;
    let mut patterns = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        match Pattern::new(trimmed) {
            Ok(pattern) => patterns.push(pattern),
            Err(e) => eprintln!("Invalid ignore pattern '{}': {}", trimmed, e),
        }
    }
    Ok(patterns)
}

/// Checks if the given path should be ignored.
fn should_ignore(path: &Path, base: &Path, ignore_patterns: &Vec<Pattern>) -> bool {
    if let Ok(relative) = path.strip_prefix(base) {
        let relative_str = relative.to_string_lossy();
        for pattern in ignore_patterns {
            if pattern.matches(&relative_str) {
                return true;
            }
        }
    }
    false
}

/// Recursively process the given path.

#[async_recursion]
async fn process_path(
    app: tauri::AppHandle,
    path: &Path,
    base: &Path,
    ignore_patterns: &Vec<Pattern>,
    zone_name: &str,
    path_to_sort: &str,
) -> io::Result<()> {
    println!("Processing path: {}", path.display());
    if should_ignore(path, base, ignore_patterns) {
        return Ok(());
    }

    if path.is_dir() {
        //println!("Processing directory: {}", path.display());
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let appc= app.clone();
            process_path(
                appc,
                &entry_path,
                base,
                ignore_patterns,
                zone_name,
                path_to_sort,
            )
            .await?;
        }
    } else if path.is_file() {
        //println!("Processing file: {}", path.display());
        ai_summarize_one_file(
            app,
            zone_name,
            base.to_str().unwrap(),
            path.to_str().unwrap(),
        ).await;
    }
    Ok(())
}

use chrono::{Local};

#[tauri::command]
pub async fn ai_summarize_one_file(
    app: tauri::AppHandle,
    zone_name: &str,
    root_path: &str,
    file_path: &str,
) -> Result<String, String> {
    println!("ai_summarize_one: {} {}", root_path, file_path);
    // Create tmp file for file_summary
    let opp=functions::system::write_to_temp_file(
        format!("{}.json", file_path),
        "{}".to_string(),
    )
    .unwrap();
println!("opp: {}", opp);
    let summarize_command = app
        .shell()
        .sidecar("summarize_files")
        .map_err(|e| e.to_string())?
        .env("GEMINI_API_KEY", system::get_api_key().unwrap())
        .args(&[
            // system prompt for sort_files (from resource folder)
            app.path()
                .resolve("resources/2_system_prompt.md", BaseDirectory::Resource)
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
            // user prompt for sort_files
            app.path()
                .resolve("resources/2_user_prompt.txt", BaseDirectory::Resource)
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
            // path to sort (i.e. the folder to be reorganized)
            root_path,
            file_path,
            opp.as_str(),
        ]);
    println!("summarize_command: {:?}", summarize_command);
    let (mut rx, _child) = summarize_command.spawn().map_err(|e| e.to_string())?;
    let task = tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line_bytes) = event {
                let line = String::from_utf8_lossy(&line_bytes);
                println!("ai_sort sidecar output: {}", line);
            }
        }
    });
    task.await.map_err(|e| e.to_string())?;
    println!("ai_sort sidecar finished");
    // Read summary output from tmp file
    let result = fs::read_to_string(
        system::wrap_tmp_dir(format!("{}.json", file_path).as_str()).unwrap()
    ).unwrap();

    // Determine file's full path
    let full_file_path = format!("{}", file_path);
    println!("full_file_path: {}", full_file_path);
    // Get file's last modified date
    let metadata = fs::metadata(&full_file_path)
        .map_err(|e| e.to_string())?;
    let modified_time = metadata.modified()
        .map_err(|e| e.to_string())?;
    let last_modified_datetime: DateTime<Local> = modified_time.into();
    let last_modified = last_modified_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("last_modified: {}", last_modified);
    // Set last summary date as today
    let today = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Get the file_id from your file system
    let fileID: u64 = functions::file::get_file_id(&full_file_path)
        .unwrap();

    // Escape values to prevent SQL injection and syntax errors.
    let escaped_result = result.replace("'", "''");
    let escaped_file_path = full_file_path.replace("'", "''");
    println!("escaped_result: {}", escaped_result);
    // Get the database connection
    let db = sql::get_db().await;
    
    // Perform an UPSERT (requires that file_id be unique)
    let sql_statement = format!(
        "INSERT INTO {} (file_path, file_id, summary, last_modified_date, last_summary_date) \
         VALUES ('{}', {}, '{}', '{}', '{}') \
         ON CONFLICT (file_id) DO UPDATE SET \
           file_path = EXCLUDED.file_path, \
           summary = EXCLUDED.summary, \
           last_modified_date = EXCLUDED.last_modified_date, \
           last_summary_date = EXCLUDED.last_summary_date;",
        format!("zone_{}", zone_name),
        escaped_file_path,
        fileID,
        escaped_result,
        last_modified,
        today
    );
    println!("sql_statement: {}", sql_statement);
    db.exec(&sql_statement)
        .await
        .unwrap();

    Ok("good".to_string())
}
