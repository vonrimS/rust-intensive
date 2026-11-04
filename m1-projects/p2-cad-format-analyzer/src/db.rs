use rusqlite::{Connection, Result, params};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MachineRecord {
    pub id: i64,
    pub name: String,
    pub max_x: f64,
    pub max_y: f64,
}

/// Parameters structure for logging inspection results, avoiding too_many_arguments warning.
pub struct InspectionParams<'a> {
    pub file_name: &'a str,
    pub file_hash: &'a str,
    pub verdict: &'a str,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub cut_length: Option<f64>,
    pub estimated_cost: Option<f64>,
    pub error_reason: Option<&'a str>,
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
                params!["Laser-Compact-01", 3000.0, 1500.0],
            )?;
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
    pub fn log_inspection(&self, params: InspectionParams<'_>) -> Result<()> {
        let query = "INSERT INTO inspection_logs (file_name, file_hash, verdict, width, height, cut_length, estimated_cost, error_reason) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
        
        self.conn.execute(
            query,
            rusqlite::params![
                params.file_name,
                params.file_hash,
                params.verdict,
                params.width,
                params.height,
                params.cut_length,
                params.estimated_cost,
                params.error_reason,
            ],
        )?;
        Ok(())
    }

    pub fn get_active_machines(&self) -> Result<Vec<MachineRecord>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, max_x, max_y FROM machines WHERE is_active = 1")?;

        let machine_iter = stmt.query_map([], |row| {
            Ok(MachineRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                max_x: row.get(2)?,
                max_y: row.get(3)?,
            })
        })?;

        let mut machines = Vec::new();
        for machine in machine_iter {
            machines.push(machine?);
        }

        Ok(machines)
    }

    pub fn get_cutting_tariff(&self, material: &str, thickness: f64) -> Result<Option<f64>> {
        let mut stmt = self.conn.prepare(
            "SELECT price_per_meter FROM cutting_tariffs WHERE LOWER(material) = LOWER(?1) AND thickness_mm = ?2 LIMIT 1"
        )?;

        let mut rows = stmt.query(params![material, thickness])?;

        if let Some(row) = rows.next()? {
            let price: f64 = row.get(0)?;
            Ok(Some(price))
        } else {
            Ok(None)
        }
    }
}
