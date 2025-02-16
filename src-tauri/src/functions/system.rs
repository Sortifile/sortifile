use std::path;
use std::env;

use dirs;
//#[tauri::command]
pub fn get_appdata_dir() -> Result<String, std::env::VarError> {
    match std::env::var("APPDATA") {
        Ok(appdata) => {
            //println!("APPDATA is: {}", appdata);
            Ok(appdata)
        }
        Err(e ) => {
            //println!("Couldn't read APPDATA: {}", e);
            Err(e)
        }
    }
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
