use std::env;
use std::path::PathBuf;

use dirs;

use crate::functions::file;
//#[tauri::command]
pub fn get_appdata_dir() -> Result<String, std::env::VarError> {
    match std::env::var("APPDATA") {
        Ok(appdata) => {
            //println!("APPDATA is: {}", appdata);
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

pub fn write_to_temp_file(file_name: String, data: String) -> std::io::Result<()> {
    // Get the temporary directory path
    let mut temp_path = get_tmp_dir().unwrap();
    // Append your desired file name
    temp_path.push_str(file_name.as_str());
    // Write text content into the file
    std::fs::write(&temp_path, data.as_str())?;
    Ok(())
}

#[tauri::command]
pub fn get_api_key() -> Result<String, std::io::Error> {
    match get_appdata_dir() {
        Ok(appdata) => {
            let api_key_path = std::path::Path::new(&appdata).join(".key");
            let api_key = std::fs::read_to_string(api_key_path);
            match api_key {
                Ok(key) => Ok(key),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}

#[tauri::command]
pub fn set_api_key(api_key: String) -> Result<(), std::io::Error> {
    match get_appdata_dir() {
        Ok(appdata) => {
            let api_key_path = std::path::Path::new(&appdata).join(".key");
            let api_key = std::fs::write(api_key_path, api_key);
            match api_key {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}
