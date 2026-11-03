use rusqlite::{Connection, Result, params};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens the SQLite database and initializes it using the external schema file,
    /// then seeds default manufacturing data if the tables are empty.
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;

        let schema_sql = include_str!("../db/schema.sql");
        conn.execute_batch(schema_sql)?;

        let db = Self { conn };
        db.seed_defaults()?;

        Ok(db)
    }

    /// Populates initial reference data (machines, default sheets, and cutting tariffs)
    /// to ensure the database is never empty on first launch.
    fn seed_defaults(&self) -> Result<()> {
        let machine_count: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM machines", [], |row| row.get(0))?;

        if machine_count == 0 {
            self.conn.execute(
                "INSERT INTO machines (name, max_x, max_y, is_active) VALUES (?1, ?2, ?3, 1)",
                params!["Laser-Fiber-01", 10000.0, 7500.0],
            )?;
        }

        let tariff_count: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM cutting_tariffs", [], |row| row.get(0))?;

        if tariff_count == 0 {
            self.conn.execute(
                "INSERT INTO cutting_tariffs (material, thickness_mm, price_per_meter) VALUES (?1, ?2, ?3)",
                params!["st3", 2.0, 45.0],
            )?;
            self.conn.execute(
                "INSERT INTO cutting_tariffs (material, thickness_mm, price_per_meter) VALUES (?1, ?2, ?3)",
                params!["st3", 3.0, 55.0],
            )?;
        }

        Ok(())
    }

    /// Inserts an inspection log record into the SQLite registry.
    pub fn log_inspection(
        &self,
        file_name: &str,
        file_hash: &str,
        verdict: &str,
        width_mm: Option<f64>,
        height_mm: Option<f64>,
        cut_length_mm: Option<f64>,
        estimated_cost: Option<f64>,
        error_reason: Option<&str>,
    ) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO inspection_logs 
            (file_name, file_hash, verdict, width_mm, height_mm, cut_length_mm, estimated_cost, error_reason)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                file_name,
                file_hash,
                verdict,
                width_mm,
                height_mm,
                cut_length_mm,
                estimated_cost,
                error_reason,
            ],
        )?;
        Ok(())
    }
}
