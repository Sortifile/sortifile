use super::sql::get_value_as_string;
use crate::functions::sql;
use serde::{Deserialize, Serialize};
use serde_json::ser;
use sqlx::{
    any,
    sqlite::{SqliteQueryResult, SqliteRow},
    sqlx_macros, Column, Error, Row, Sqlite, SqlitePool,
};
use tauri::utils::config;

#[derive(Debug, Serialize, Deserialize)]
struct Zone {
    zone_name: String,
    root_path: String,
}

#[tauri::command]
pub async fn create_zone(
    zone_name: &str,
    root_path: &str,
    config: String,
    ignore: String,
    rules: &str,
) -> Result<String, String> {
    let db = sql::get_db().await;
    // Create zone table with the additional file_id column.
    db.exec(
        format!(
            "CREATE TABLE IF NOT EXISTS zone_{} (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        file_path TEXT NOT NULL,
        file_id INTEGER UNIQUE,
        summary TEXT,
        last_modified_date TEXT,
        last_summary_date TEXT
    );",
            zone_name
        )
        .as_str(),
    )
    .await
    .unwrap();
    db.exec(
        format!(
            "INSERT INTO zone_list (zone_name, root_path, zone_rules) VALUES ('{}', '{}', '{}');",
            zone_name,
            root_path,
            rules.replace("'", "''")
        )
        .as_str(),
    )
    .await
    .unwrap();
    std::fs::write(format!("{}/.sortifile.conf", root_path), config).unwrap();
    std::fs::write(format!("{}/.sortifile-ignore", root_path), ignore).unwrap();
    // dfs and add all files to the database
    Ok("Ok".to_string())
}

#[tauri::command]
pub async fn get_zone_list() -> String {
    let db = sql::get_db().await;
    let zone_list = db
        .exec_select("SELECT zone_name, root_path FROM zone_list;")
        .await
        .unwrap();
    let mut zones = Vec::new();
    for row in zone_list {
        let mut zone = Zone {
            zone_name: "".to_string(),
            root_path: "".to_string(),
        };
        for (i, column) in row.columns().iter().enumerate() {
            let col_name = column.name();
            let value_str = sql::get_value_as_string(&row, i);
            if col_name == "zone_name" {
                zone.zone_name = value_str;
            } else if col_name == "root_path" {
                zone.root_path = value_str;
            }
        }
        zones.push(zone);
    }
    let serialized_zones = serde_json::to_string_pretty(&zones).unwrap();
    serialized_zones
}

#[tauri::command]
pub async fn get_zone_rules(zone_name: &str) -> Result<String, String> {
    let db = sql::get_db().await;
    let zone_rules = db
        .exec_select(
            format!(
                "SELECT zone_rules FROM zone_list WHERE zone_name='{}';",
                zone_name
            )
            .as_str(),
        )
        .await
        .unwrap();
    let rule: String = get_value_as_string(&zone_rules[0], 0);
    println!("rule: {}", rule);
    Ok(rule)
}

#[tauri::command]
pub async fn set_zone_rules(zone_name: &str, rules: &str) -> Result<(), String> {
    // get project file

    let db = sql::get_db().await;
    // replace quotation marks with double quotation marks
    let rules = rules.replace("'", "''");

    db.exec(
        format!(
            "UPDATE zone_list SET zone_rules='{}' WHERE zone_name='{}';",
            rules, zone_name
        )
        .as_str(),
    )
    .await
    .unwrap();
    Ok(())
}

#[tauri::command]
pub async fn delete_zone(zone_name: &str) -> Result<(), String> {
    let db = sql::get_db().await;
    db.exec(format!("DROP TABLE zone_{};", zone_name).as_str())
        .await
        .unwrap();
    db.exec(format!("DELETE FROM zone_list WHERE zone_name='{}';", zone_name).as_str())
        .await
        .unwrap();
    let zone_list = db
        .exec_select(
            format!(
                "SELECT root_path FROM zone_list WHERE zone_name='{}';",
                zone_name
            )
            .as_str(),
        )
        .await
        .unwrap();
    let root_path = get_value_as_string(&zone_list[0], 0);
    std::fs::remove_file(format!("{}/.sortifile.conf", root_path)).unwrap();
    Ok(())
}
