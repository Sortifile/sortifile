use sqlx::{
    sqlite::{SqliteQueryResult, SqliteRow},
    sqlx_macros, Column, Error, Row, Sqlite, SqlitePool,
};
use std::sync::Arc;

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
        let file_id = crate::functions::file::get_file_id(file_path).unwrap();
        let query = format!(
            "SELECT summary FROM {} WHERE file_id = {};",
            table_name, file_id
        );

        let row = sqlx::query(&query)
            .bind(file_path)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let summary: String = row.get(0);
            Ok(summary)
        } else {
            Ok("FFAAIILLEEDD".to_string())
        }
    }
    pub async fn checkresum(&self, zone_name: &str, file_path: &str) -> Result<bool, Error> {
        let table_name = format!("zone_{}", zone_name);
        let file_id = crate::functions::file::get_file_id(file_path).unwrap();
        // select last_modified_date, last_summary_date from zone_name where file_id = file_id
        let query = format!(
            "SELECT last_modified_date, last_summary_date FROM {} WHERE file_id = {};",
            table_name, file_id
        );
        let row = sqlx::query(&query)
            .bind(file_path)
            .fetch_optional(&self.pool)
            .await?;
        // return true is no rows found

        // return true if last_modified_date > last_summary_date
        if let Some(row) = row {
            let last_modified_date: String = row.get(0);
            let last_summary_date: String = row.get(1);
            if last_modified_date > last_summary_date {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else {
            return Ok(true);
        }
    }

    pub async fn exec_select(&self, input: &str) -> Result<Vec<SqliteRow>, Error> {
        // replace quotations with double quotations
        println!("Executing query: {}", input);
        let rows = sqlx::query(&input).fetch_all(&self.pool).await;
        Ok(rows.unwrap())
    }

    pub async fn exec(&self, input: &str) -> Result<SqliteQueryResult, Error> {
        // replace quotations with double quotations
        println!("Executing query: {}", input);
        let changes = sqlx::query(&input).execute(&self.pool).await;
        Ok(changes.unwrap())
    }
}

pub async fn get_db() -> Database {
    let db_url = "sqlite://my_database.db";
    let pool = Database::new(db_url).await;
    pool
}
