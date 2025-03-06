use crate::functions::sql;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::{
    any,
    sqlite::{SqliteQueryResult, SqliteRow},
    sqlx_macros, Column, Error, Row, Sqlite, SqlitePool,
};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tauri::async_runtime;
#[derive(Debug, Serialize, Deserialize)]
struct move_history {
    src_path: String,
    new_path: String,
    move_timestamp: String,
    moved_by: String,
    reason: String,
}
#[tauri::command]
pub async fn get_summary_of_one_file(
    zone_name: &str,
    file_path: &str,
) -> Result<String, String> {
    println!("zone_name: {}", zone_name);
    println!("file_path: {}", file_path);
    let db = sql::get_db().await;
    let summary = db.get_file_summary(zone_name, file_path).await.unwrap();
    println!("summary: {}", summary);
    Ok(summary)
}
#[tauri::command]
pub async fn set_summary_of_one_file(
    zone_name: &str,
    file_path: &str,
    summary: &str,
) -> Result<(), String> {
    let db = sql::get_db().await;
    let table_name = format!("zone_{}", zone_name);
    db.exec(
        format!(
            "UPDATE {}
SET summary = 'New summary text',  
last_summary_date = CURRENT_TIMESTAMP   
WHERE file_path = {}
  AND zone = 'your_zone';       
",
            table_name, file_path
        )
        .as_str(),
    )
    .await
    .unwrap();
    Ok(())
}

#[tauri::command]
pub async fn get_move_history(num: u64) -> Result<String, String> {
    let db = sql::get_db().await;
    let result = db
        .exec_select(format!("SELECT * FROM move_history LIMIT {};", num).as_str())
        .await
        .unwrap();
    let mut move_histories = Vec::new();
    for row in result {
        let mut move_history = move_history {
            src_path: "".to_string(),
            new_path: "".to_string(),
            move_timestamp: "".to_string(),
            moved_by: "".to_string(),
            reason: "".to_string(),
        };
        for (i, column) in row.columns().iter().enumerate() {
            let col_name = column.name();
            let value_str = sql::get_value_as_string(&row, i);
            if col_name == "move_timestamp" {
                move_history.move_timestamp = value_str;
            } else if col_name == "src_path" {
                move_history.src_path = value_str;
            } else if col_name == "new_path" {
                move_history.new_path = value_str;
            } else if col_name == "moved_by" {
                move_history.moved_by = value_str;
            } else if col_name == "reason" {
                move_history.reason = value_str;
            }
        }
        move_histories.push(move_history);
    }
    let serialized_move_histories = serde_json::to_string_pretty(&move_histories).unwrap();
    Ok(serialized_move_histories)
}

#[tauri::command]
pub async fn get_move_history_with_fileid(num: u64, fileID: String) -> Result<String, sqlx::Error> {
    let db = sql::get_db().await;
    let result = db
        .exec_select(
            format!(
                "SELECT * FROM move_history LIMIT {} WHERE fileID={};",
                num, fileID
            )
            .as_str(),
        )
        .await
        .unwrap();
    let mut move_histories = Vec::new();
    for row in result {
        let mut move_history = move_history {
            src_path: "".to_string(),
            new_path: "".to_string(),
            move_timestamp: "".to_string(),
            moved_by: "".to_string(),
            reason: "".to_string(),
        };
        for (i, column) in row.columns().iter().enumerate() {
            let col_name = column.name();
            let value_str = sql::get_value_as_string(&row, i);
            if col_name == "move_timestamp" {
                move_history.move_timestamp = value_str;
            } else if col_name == "src_path" {
                move_history.src_path = value_str;
            } else if col_name == "new_path" {
                move_history.new_path = value_str;
            } else if col_name == "moved_by" {
                move_history.moved_by = value_str;
            } else if col_name == "reason" {
                move_history.reason = value_str;
            }
        }
        move_histories.push(move_history);
    }
    let serialized_move_histories = serde_json::to_string_pretty(&move_histories).unwrap();
    Ok(serialized_move_histories)
}

#[tauri::command]
pub async fn move_file(
    zone_path: &str,
    src_path: &str,
    new_path: &str,
    moved_by: &str,
    reason: &str,
) -> Result<(), String> {
    let db = sql::get_db().await;
    //replace src and new from relative to absolute by adding zone_path
    let abs_src_path = format!("{}/{}", zone_path, src_path);
    let abs_new_path = format!("{}/{}", zone_path, new_path);
    // create directory if it doesn't exist
    let new_path_p = Path::new(abs_new_path.as_str());
    if let Some(parent) = new_path_p.parent() {
        fs::create_dir_all(parent);
    }
    fs::rename(&abs_src_path, &abs_new_path).unwrap();
    /* 
    db.exec(
        format!(
            "INSERT INTO move_history (zone_path, src_path, new_path, moved_by, reason) VALUES ('{}', '{}', '{}', '{}', '{}');",
            zone_path, abs_src_path, abs_new_path, moved_by, reason
        )
        .as_str(),
    ).await.unwrap();*/
    Ok(())
}

#[tauri::command]
pub fn get_ignore_list(zone_path: &str) -> Result<String, String> {
    println!("zone_path: {}", zone_path);
    let ignore_list = fs::read_to_string(format!("{}/.sortifile-ignore", zone_path));
    match ignore_list {
        Ok(list) => {
            println!("ignore_list: {}", list);
            Ok(list)},
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn set_ignore_list(zone_path: &str, ignore_list: &str) -> Result<(), String> {
    fs::write(format!("{}/.sortifile-ignore", zone_path), ignore_list);
    Ok(())
}

#[tauri::command]
pub fn get_project_file(zone_path: &str) -> Result<String, String> {
    let project_file = fs::read_to_string(format!("{}/.sortifile.conf", zone_path));
    match project_file {
        Ok(file) => {
            println!("project_file: {}", file);
            Ok(file)},
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn set_project_file(zone_path: &str, project_file: &str) -> Result<(), String> {
    fs::write(format!("{}/.sortifile.conf", zone_path), project_file);
    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct FileNode {
    name: String,
    path: String,
    file_id: u64,
    is_directory: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<FileNode>>,
}

/// Recursively builds a `FileNode` for the given path.
fn build_file_node(path: &Path) -> io::Result<FileNode> {
    let metadata = fs::metadata(path)?;
    let is_directory = metadata.is_dir();
    let name = path
        .file_name()
        .map(|os_str| os_str.to_string_lossy().into_owned())
        // For the root path, use the whole path string.
        .unwrap_or_else(|| path.to_string_lossy().into_owned());

    let path_str = path.to_string_lossy().into_owned();
    println!("path_str: {}", path_str);
    let file_id: u64 = get_file_id(&path.to_string_lossy()).unwrap();
    let children = if is_directory {
        // Read the directory entries and recursively build child nodes.
        let mut nodes = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();
            nodes.push(build_file_node(&child_path)?);
        }
        Some(nodes)
    } else {
        None
    };

    Ok(FileNode {
        name,
        path: path_str,
        file_id: file_id,
        is_directory,
        children,
    })
}

fn gen_file_tree(root: &Path) -> io::Result<Vec<FileNode>> {
    let mut nodes = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        nodes.push(build_file_node(&entry.path())?);
    }
    Ok(nodes)
}

#[tauri::command]
pub fn get_file_tree(zone_path: String) -> Result<String, String> {
    let path = PathBuf::from(zone_path);
    let file_tree = gen_file_tree(&path).unwrap();
    let json =
        serde_json::to_string_pretty(&file_tree).expect("Failed to serialize file tree to JSON");
    Ok(json)
}
use std::fs::File;
use std::os::windows::io::AsRawHandle;
use winapi::um::fileapi::BY_HANDLE_FILE_INFORMATION;
use winapi::um::fileapi::GetFileInformationByHandle;

#[tauri::command]
pub fn get_file_id(file_path: &str) -> Result<u64, String> {
    //pass if the path is a dir
    if Path::new(file_path).is_dir() {
        return Ok(0);
    }
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    // Cast the handle from std::ffi::c_void to winapi::ctypes::c_void
    let handle = file.as_raw_handle() as *mut winapi::ctypes::c_void;

    unsafe {
        let mut info: BY_HANDLE_FILE_INFORMATION = std::mem::zeroed();
        if GetFileInformationByHandle(handle, &mut info) == 0 {
            return Err("Failed to get file information".to_string());
        }
        // Combine the high and low parts of the file index.
        let file_index = ((info.nFileIndexHigh as u64) << 32) | (info.nFileIndexLow as u64);
        Ok(file_index)
    }
}