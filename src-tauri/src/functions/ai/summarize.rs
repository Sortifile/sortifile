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
pub async fn ai_summarize_all(
    app: &tauri::AppHandle,
    zone_name: &str,
    zone_path: &str,
    path_to_sort: &str,
) -> Result<String, String> {
    let db = sql::get_db().await;
    process_directory_recursively(&app, zone_path.to_string(), zone_name, path_to_sort)
        .await
        .unwrap();
    Ok("good".to_string())
}

/// The rest of the functions below are helper functions for processing directories and files.

#[command]
async fn process_directory_recursively(
    app: &tauri::AppHandle,
    dir_path: String,
    zone_name: &str,
    path_to_sort: &str,
) -> Result<(), String> {
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
    app: &tauri::AppHandle,
    path: &Path,
    base: &Path,
    ignore_patterns: &Vec<Pattern>,
    zone_name: &str,
    path_to_sort: &str,
) -> io::Result<()> {
    if should_ignore(path, base, ignore_patterns) {
        return Ok(());
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            process_path(
                &app,
                &entry_path,
                base,
                ignore_patterns,
                zone_name,
                path_to_sort,
            )
            .await?;
        }
    } else if path.is_file() {
        ai_summarize_one(
            app,
            zone_name,
            base.to_str().unwrap(),
            path.to_str().unwrap(),
        );
    }
    Ok(())
}

#[tauri::command]
pub async fn ai_summarize_one(
    app: &tauri::AppHandle,
    zone_name: &str,
    root_path: &str,
    file_path: &str,
) -> Result<String, String> {
    // create tmp file for file_summary
    functions::system::write_to_temp_file(
        format!("{}_{}.json", root_path, file_path),
        "".to_string(),
    )
    .unwrap();
    let summarize_command = app
        .shell()
        .sidecar("summarize_files")
        .map_err(|e| e.to_string())?
        .args(&[
            // system prompt for sort_files (from resource folder)
            app.path()
                .resolve(
                    "resources/2_system_prompt.json",
                    BaseDirectory::Resource,
                )
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
            // user prompt for sort_files
            app.path()
                .resolve(
                    "resources/2_user_prompt.json",
                    BaseDirectory::Resource,
                )
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
            // path to sort (i.e. the folder to be reorganized)
            root_path,
            file_path,
        ]);
    let (mut rx, _child) = summarize_command.spawn().map_err(|e| e.to_string())?;
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line_bytes) = event {
                let line = String::from_utf8_lossy(&line_bytes);
                println!("ai_sort sidecar output: {}", line);
            }
        }
    });
    // read from move_steps file to string
    let result = fs::read_to_string(system::wrap_tmp_dir(format!("{}_{}.json", root_path, file_path).as_str()).unwrap()).unwrap();
    // get db
    let db = sql::get_db().await;
    let fileID: u64 =
        functions::file::get_file_id(format!("{}/{}", root_path, file_path).as_str()).unwrap();
    db.exec(&format!(
        "UPDATE {} SET summary = {} WHERE fileID = {};",
        format!("zone_{}", zone_name),
        result,
        fileID
    ))
    .await
    .unwrap();
    Ok("good".to_string())
}
