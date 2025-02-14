use tauri::async_runtime;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::functions::sql;
use sqlx::{any, sqlite::{SqliteQueryResult, SqliteRow}, sqlx_macros, Column, Error, Row, Sqlite, SqlitePool};


#[derive(Debug, Serialize, Deserialize)]
struct move_history {
    zone_path: String,
    src_path: String,
    new_path: String,
    moved_by: String,
    reason: String,
}

#[tauri::command]
pub async fn get_summary_of_one_file(zone_name: &str, file_path: &str) -> Result<String, sqlx::Error> {
    let db = sql::get_db().await;
    let summary = db.get_file_summary(zone_name, file_path).await.unwrap();
    Ok(summary)
}

#[tauri::command]
pub async fn set_summary_of_one_file(zone_name: &str, file_path: &str, summary: &str) -> Result<(), sqlx::Error>{
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
pub async fn get_move_history(num: u64) -> Result<String, sqlx::Error>{
    let db = sql::get_db().await;
    let result=db.exec_select(format!("SELECT * FROM move_history LIMIT {};", num).as_str()).await.unwrap();
    let mut move_histories = Vec::new();
    for row in result {
        let mut move_history = move_history {
            zone_path: "".to_string(),
            src_path: "".to_string(),
            new_path: "".to_string(),
            moved_by: "".to_string(),
            reason: "".to_string(),
        };
        for (i, column) in row.columns().iter().enumerate() {
            let col_name = column.name();
            let value_str = sql::get_value_as_string(&row, i);
            if col_name == "zone_path" {
                move_history.zone_path = value_str;
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
pub async fn move_file(zone_path: &str, src_path: &str, new_path: &str, moved_by: &str, reason: &str) -> Result<(), sqlx::Error>{
    let db = sql::get_db().await;
    fs::rename(src_path, new_path).unwrap();
    db.exec(
        format!(
            "INSERT INTO move_history (zone_path, src_path, new_path, moved_by, reason) VALUES ('{}', '{}', '{}', '{}', '{}');",
            zone_path, src_path, new_path, moved_by, reason
        )
        .as_str(),
    );
    Ok(())
}