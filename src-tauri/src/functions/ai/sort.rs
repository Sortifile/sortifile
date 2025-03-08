use crate::functions;
use crate::functions::ai::utils;
use crate::functions::file;
use crate::functions::sql;
use crate::functions::system::get_appdata_dir;
use crate::functions::zone;
use crate::functions::system;
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
pub async fn ai_sort(
    app: tauri::AppHandle,
    zone_name: &str,
    zone_path: &str,
    path_to_sort: &str,
) -> Result<String, String> {
    // Step 1: Update file summaries.
    // Here you would update the summaries so that files under path_to_sort have
    // their `allow_move` field set to true and others to false.
    // For example:
    // functions::file::update_allow_move_flag(zone_name, path_to_sort);
    //
    // (Implement the above function as needed.)

    // Step 2: Call the Python sidecar for sorting.
    let db = sql::get_db().await;
    let mut file_summary: Vec<String> = Vec::new();
    let mut history_file_movements: Vec<String> = Vec::new();

    process_directory_recursively(
        zone_path.to_string(),
        zone_name,
        path_to_sort,
        &mut file_summary,
        &mut history_file_movements,
    )
    .await
    .unwrap();
    // serialize file_summary to a string
    let file_summary_str = serde_json::to_string(&file_summary).unwrap();
    let history_file_movement_str = serde_json::to_string(&history_file_movements).unwrap();

    // write file_summary to a temporary file
    functions::system::write_to_temp_file(
        format!("zone_{}_file_summary_tmp.json", zone_name),
        file_summary_str,
    )
    .unwrap();
    functions::system::write_to_temp_file(
        format!("zone_{}_history_file_movements_tmp.json", zone_name),
        history_file_movement_str,
    )
    .unwrap();
    let sort_command = app
        .shell()
        .sidecar("sort_files")
        .map_err(|e| e.to_string())?
        .env("GEMINI_API_KEY", system::get_api_key().unwrap())
        .args(&[
            // system prompt for sort_files (from resource folder)
            app.path()
                .resolve(
                    "resources/3_system_prompt.md",
                    BaseDirectory::Resource,
                )
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
            // user prompt for sort_files
            app.path()
                .resolve(
                    "resources/3_user_prompt.txt",
                    BaseDirectory::Resource,
                )
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
            // path to sort (i.e. the folder to be reorganized)
            path_to_sort,
            // rule file
            format!("{}/.sortifile.conf", zone_path).as_str(),
            // file summary file (should be prepared by your logic)
            system::wrap_tmp_dir(format!("zone_{}_file_summary_tmp.json", zone_name).as_str()).unwrap().as_str(),
            // history file movements file
            system::wrap_tmp_dir(format!("zone_{}_history_file_movements_tmp.json", zone_name).as_str()).unwrap().as_str(),
            // output file where move steps are written
            system::wrap_tmp_dir(format!("zone_{}_move_steps_tmp.json", zone_name).as_str()).unwrap().as_str(),
        ]);
    println!("ai_sort command: {:?}", sort_command);
    let (mut rx, _child) = sort_command.spawn().map_err(|e| e.to_string())?;
    let task = tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line_bytes) = event {
                let line = String::from_utf8_lossy(&line_bytes);
                println!("ai_sort sidecar output: {}", line);
            }
        }
    });
    task.await.map_err(|e| e.to_string())?;

    // read from move_steps file to string
    let result = fs::read_to_string(system::wrap_tmp_dir(format!("zone_{}_move_steps_tmp.json", zone_name).as_str()).unwrap()).unwrap();
    println!("ai_sort result: {}", result);
    Ok(result)
}

/// The rest of the functions below are helper functions for processing directories and files.

#[command]
async fn process_directory_recursively(
    dir_path: String,
    zone_name: &str,
    path_to_sort: &str,
    file_summary: &mut Vec<String>,
    history_file_movements: &mut Vec<String>,
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
    process_path(
        base,
        base,
        &ignore_patterns,
        zone_name,
        path_to_sort,
        file_summary,
        history_file_movements,
    )
    .await;
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
    // return true if file name starts with a dot
    if path.file_name().unwrap().to_str().unwrap().starts_with(".") {
        return true;
    }
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
    path: &Path,
    base: &Path,
    ignore_patterns: &Vec<Pattern>,
    zone_name: &str,
    path_to_sort: &str,
    file_summary: &mut Vec<String>, // mutable vector
    history_file_movements: &mut Vec<String>,
) -> io::Result<()> {
    if should_ignore(path, base, ignore_patterns) {
        return Ok(());
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            process_path(
                &entry_path,
                base,
                ignore_patterns,
                zone_name,
                path_to_sort,
                file_summary,
                history_file_movements,
            )
            .await?;
        }
    } else if path.is_file() {
        println!("Processing file: {}", path.to_string_lossy());
        // Process each file by summarizing it.
        // Note: Any error from get_summary_of_one_file is unwrapped for brevity.
        let summary = functions::file::get_summary_of_one_file(zone_name, path.to_str().unwrap())
            .await
            .unwrap();
        let fileID = functions::file::get_file_id(path.to_str().unwrap()).unwrap();
        // Append missing fields using the file path and the given sort path.
        let base_str = base.to_str().unwrap();
        let updated_summary =
            append_missing_fields_with_path(base_str, summary.as_str(), path.to_str().unwrap(), path_to_sort)
                .unwrap();
        file_summary.push(updated_summary);
    }
    Ok(())
}

/// Returns a JSON object containing file metadata.
/// Attempts to get the file's creation and modification times.
/// If not available, returns empty strings.
fn get_file_metadata(path: &str) -> Value {
    let metadata = fs::metadata(path);
    match metadata {
        Ok(meta) => {
            let created = meta
                .created()
                .ok()
                .and_then(|t| Some(DateTime::<Utc>::from(t).to_rfc3339()))
                .unwrap_or_else(|| "".to_string());
            let modified = meta
                .modified()
                .ok()
                .and_then(|t| Some(DateTime::<Utc>::from(t).to_rfc3339()))
                .unwrap_or_else(|| "".to_string());
            json!({
                "created_date": created,
                "last_modified_date": modified,
                "last_sorted_date": "",
                "last_summarized_date": ""
            })
        }
        Err(_) => {
            json!({
                "created_date": "",
                "last_modified_date": "",
                "last_sorted_date": "",
                "last_summarized_date": ""
            })
        }
    }
}

/// Takes a JSON string and a file path. If the JSON (object or objects)
/// is missing the "src_path", "allow_move", or "metadata" fields,
/// the function appends them automatically. In particular, "src_path"
/// will be set to the provided `path` argument and metadata is gathered from the file system.
fn append_missing_fields_with_path(
    base: &str,
    json_str: &str,
    path: &str,
    path_to_sort: &str,
) -> Result<String, serde_json::Error> {

    // Parse the input JSON string.
    let mut data: Value = serde_json::from_str(json_str)?;

    // Determine the value for allow_move.
    let chc: bool = path.starts_with(path_to_sort);

    // relative path and remove leading  backslash  
    let rel_path = path.strip_prefix(base).unwrap().trim_start_matches('\\');

    // Closure to update a single summary object.
    let update_summary = |summary: &mut Value, file_path: &str| {
        // If "src_path" is missing, set it to the provided file_path.
        if summary.get("src_path").is_none() {
            summary["src_path"] = Value::String(file_path.to_string());
        }
        // If "allow_move" is missing, set it to the value of chc.
        if summary.get("allow_move").is_none() {
            summary["allow_move"] = Value::Bool(chc);
        }
        // If "metadata" is missing, get metadata from file system.
        if summary.get("metadata").is_none() {
            summary["metadata"] = get_file_metadata(file_path);
        }
    };

    // Handle either a single object or an array of objects.
    match data {
        Value::Array(ref mut arr) => {
            for summary in arr.iter_mut() {
                if summary.is_object() {
                    update_summary(summary, rel_path);
                }
            }
        }
        Value::Object(ref mut _obj) => {
            update_summary(&mut data, rel_path);
        }
        _ => {
            // If it's not an object or array, do nothing.
        }
    }

    // Serialize the updated JSON back to a pretty-printed string.
    serde_json::to_string_pretty(&data)
}

// Example usage:
