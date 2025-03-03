use sqlx::{
    sqlite::{SqliteQueryResult, SqliteRow},
    sqlx_macros, Column, Error, Row, Sqlite, SqlitePool,
};
use std::sync::Arc;
use tauri::State;
use tokio::time::{self, Duration};

pub struct Database {
    pub pool: sqlx::SqlitePool,
}

pub fn get_value_as_string(row: &SqliteRow, index: usize) -> String {
    // Try to get the column as an Option<String>.
    if let Ok(opt) = row.try_get::<Option<String>, _>(index) {
        if let Some(val) = opt {
            return val;
        }
    }
    // Try to get the column as an Option<i64> (for integer values).
    if let Ok(opt) = row.try_get::<Option<i64>, _>(index) {
        if let Some(val) = opt {
            return val.to_string();
        }
    }
    // Try to get the column as an Option<f64> (for float values).
    if let Ok(opt) = row.try_get::<Option<f64>, _>(index) {
        if let Some(val) = opt {
            return val.to_string();
        }
    }
    // Try to get the column as an Option<Vec<u8>> (for blob values).
    if let Ok(opt) = row.try_get::<Option<Vec<u8>>, _>(index) {
        if let Some(val) = opt {
            return format!("<BLOB: {} bytes>", val.len());
        }
    }
    // Fallback: if none of the above work, we return "NULL".
    "NULL".to_owned()
}

impl Database {
    pub async fn new(database_url: &str) -> Self {
        let pool = sqlx::SqlitePool::connect(database_url)
            .await
            .expect("Failed to create SQLite pool");
        Self { pool }
    }

    /// Creates a zone table with an additional file_id column.
    pub async fn create_zone_table(&self, zone_name: &str) -> Result<(), sqlx::Error> {
        let table_name = format!("zone_{}", zone_name);
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path TEXT NOT NULL,
                file_id INTEGER,
                summary TEXT,
                last_modified_date TEXT,
                last_summary_date TEXT
            );",
            table_name
        );
        sqlx::query(&query).execute(&self.pool).await?;
        Ok(())
    }

    pub fn get_pool(&self) {
        println!(
            "Database pool is active at memory address: {:p}",
            &self.pool
        );
    }

    pub async fn listen_for_changes(&self) {
        loop {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if input.contains("SELECT") {
                let rows = sqlx::query(&input).fetch_all(&self.pool).await;
                for row in rows.unwrap() {
                    // Iterate over each column in the row.
                    for (i, column) in row.columns().iter().enumerate() {
                        let col_name = column.name();
                        let value_str = get_value_as_string(&row, i);
                        println!("{}: {}", col_name, value_str);
                    }
                    println!("-------------------------");
                }
            } else {
                let changes = sqlx::query(&input).execute(&self.pool).await;
                match changes {
                    Ok(change) => {
                        println!("Change detected: {:?}", change);
                    }
                    Err(e) => {
                        eprintln!("Error checking for changes: {}", e);
                    }
                }
            }
        }
    }

    pub async fn get_file_summary(
        &self,
        zone_name: &str,
        file_path: &str,
    ) -> Result<String, Error> {
        let table_name = format!("zone_{}", zone_name);
        let query = format!("SELECT summary FROM {} WHERE file_path = ?;", table_name);
        let row = sqlx::query(&query)
            .bind(file_path)
            .fetch_one(&self.pool)
            .await?;
        let summary: String = row.get(0);
        Ok(summary)
    }

    pub async fn exec_select(&self, input: &str) -> Result<Vec<SqliteRow>, Error> {
        let rows = sqlx::query(&input).fetch_all(&self.pool).await;
        Ok(rows.unwrap())
    }

    pub async fn exec(&self, input: &str) -> Result<SqliteQueryResult, Error> {
        let changes = sqlx::query(&input).execute(&self.pool).await;
        Ok(changes.unwrap())
    }
}

pub async fn get_db() -> Database {
    let db_url = "sqlite://my_database.db";
    let pool = Database::new(db_url).await;
    pool
}
