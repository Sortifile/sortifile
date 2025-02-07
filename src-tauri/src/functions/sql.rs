use sqlx::{SqlitePool, Error};
use tauri::State;
use std::sync::Arc;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Self {
        let pool = SqlitePool::connect(database_url)
            .await
            .expect("Failed to create SQLite pool");

        Self { pool }
    }

    pub async fn create_zone_table(&self, zone_name: &str) -> Result<(), Error> {
        let table_name = format!("zone_{}", zone_name);
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path TEXT NOT NULL,
                summary TEXT,
                last_modified_date TEXT,
                last_summary_date TEXT
            );",
            table_name
        );

        sqlx::query(&query)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
