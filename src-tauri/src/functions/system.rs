use std::env;
use std::path::PathBuf;

use dirs;

use crate::functions::file;
//#[tauri::command]
pub fn get_appdata_dir() -> Result<String, std::env::VarError> {
    match std::env::var("APPDATA") {
        Ok(mut appdata) => {
            //println!("APPDATA is: {}", appdata);
            //append appdata with sortifile
            appdata.push_str("/sortifile/");
            Ok(appdata)
        }
        Err(e) => {
            //println!("Couldn't read APPDATA: {}", e);
            Err(e)
        }
    }
}

pub fn get_tmp_dir() -> Result<String, String> {
    let temp_path: PathBuf = env::temp_dir(); // 跨平台獲取 temp 目錄

    match temp_path.to_str() {
        Some(path) => Ok(path.to_string()),
        None => Err("Failed to retrieve temp directory".to_string()),
    }
}
use std::path::Path;

pub fn wrap_tmp_dir(rel: &str) -> Result<String, String> {
    let path = Path::new(rel);
    let ptt = path
        .file_name() // Option<&OsStr>
        .and_then(|name| name.to_str()) // Option<&str>
        .unwrap_or("default.txt");
    let mut temp_path = get_tmp_dir().unwrap();
    temp_path.push_str("sortifile\\");

    temp_path.push_str(ptt);
    println!("wtemp_path: {}", temp_path);
    Ok(temp_path)
}

pub fn write_to_temp_file(file_name: String, data: String) -> Result<String, String> {
    // Get the temporary directory path
    let pt = Path::new(&file_name);
    let ptt = pt
        .file_name() // Option<&OsStr>
        .and_then(|name| name.to_str()) // Option<&str>
        .unwrap_or("default.txt"); // fallback value if conversion fails
                                   // Write text content into the file
    let mut temp_path = get_tmp_dir().unwrap();
    // Append your desired file name
    temp_path.push_str("sortifile\\");
    temp_path.push_str(ptt);
    //print the path
    println!("temp_path: {}", temp_path);
    let path = Path::new(&temp_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent);
    }
    std::fs::write(&path, data.as_str());
    Ok(temp_path.to_string())
}

#[tauri::command]
pub fn get_api_key() -> Result<String, String> {
    match get_appdata_dir() {
        Ok(appdata) => {
            let api_key_path = std::path::Path::new(&appdata).join(".key");
            println!("api_key_path: {}", api_key_path.display());
            let api_key = std::fs::read_to_string(api_key_path);
            match api_key {
                Ok(key) => Ok(key),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err("Oops".to_string()),
    }
}

#[tauri::command]
pub fn set_api_key(api_key: String) -> Result<(), String> {
    match get_appdata_dir() {
        Ok(appdata) => {
            let api_key_path = std::path::Path::new(&appdata).join(".key");
            let api_key = std::fs::write(api_key_path, api_key);
            match api_key {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err("Oops".to_string()),
    }
}
