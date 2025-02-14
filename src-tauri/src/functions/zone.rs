use crate::functions::sql;
use serde_json::ser;
use sqlx::{any, sqlite::{SqliteQueryResult, SqliteRow}, sqlx_macros, Column, Error, Row, Sqlite, SqlitePool};

use serde::{Serialize, Deserialize};

use super::sql::get_value_as_string;

#[derive(Debug, Serialize, Deserialize)]
struct Zone {
    zone_name: String,
    root_path: String,
}

#[tauri::command]
pub async fn create_zone(zone_name: &str, root_path: &str) -> Result<(), sqlx::Error> {
    let db=sql::get_db().await;
    db.exec(format!("CREATE TABLE IF NOT EXISTS zone_{} (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        file_path TEXT NOT NULL,
        summary TEXT,
        last_modified_date TEXT,
        last_summary_date TEXT
    );", zone_name).as_str()).await.unwrap();
    db.exec(format!("INSERT INTO zone_list (zone_name, root_path) VALUES ('{}', '{}');", zone_name, root_path).as_str()).await.unwrap();
    // dfs and add all files to the database
    Ok(())
}

#[tauri::command]
pub async fn get_zone_list() -> String {
    let db = sql::get_db().await;
    let zone_list = db.exec_select("SELECT zone_name, root_path FROM zone_list;").await.unwrap();
    let mut zones = Vec::new();
    for row in zone_list {
        let mut zone = Zone { zone_name: "".to_string(), root_path: "".to_string() };
        for (i, column) in row.columns().iter().enumerate() {
            let col_name = column.name();
            let value_str = sql::get_value_as_string(&row, i);
            if col_name=="zone_name" {
                zone.zone_name=value_str;
            }else if col_name=="root_path" {
                zone.root_path=value_str;
            }
        }

        zones.push(zone);
    }   
    let serialized_zones = serde_json::to_string_pretty(&zones).unwrap();
    serialized_zones
}

#[tauri::command]
pub async fn get_zone_rules(zone_name: &str) -> Result<String, Error> {
    let db = sql::get_db().await;
    let zone_rules = db.exec_select(format!("SELECT zone_rules FROM zone_list WHERE zone_name={};", zone_name).as_str()).await.unwrap();
    let rule:String = get_value_as_string(&zone_rules[0], 3);
    Ok(rule)
}


#[tauri::command]
pub async fn set_zone_rules(zone_name: &str, rules: &str) -> Result<(), Error> {
    let db = sql::get_db().await;
    db.exec(format!("UPDATE zone_list SET zone_rules='{}' WHERE zone_name='{}';", rules, zone_name).as_str()).await.unwrap();
    Ok(())
}

