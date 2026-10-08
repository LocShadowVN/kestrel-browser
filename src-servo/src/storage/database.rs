use rusqlite::{params, Connection};
use shared::{BookmarkRecord, HistoryRecord};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn init() -> Self {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("kestrel-browser");
        fs::create_dir_all(&data_dir).expect("Cannot create data dir");

        let db_path = data_dir.join("kestrel.sqlite");
        let conn = Connection::open(&db_path).expect("SQLite open failed");

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL,
                title TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS bookmarks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL UNIQUE,
                title TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_history_url ON history(url);
            ",
        )
        .expect("Schema creation failed");

        log::info!("Database initialized at {:?}", db_path);

        Self {
            conn: Mutex::new(conn),
        }
    }

    pub fn insert_history(&self, url: &str, title: &str) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO history (url, title) VALUES (?1, ?2)",
            params![url, title],
        )?;
        Ok(())
    }

    pub fn fetch_history(&self, limit: usize) -> rusqlite::Result<Vec<HistoryRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, url, title, timestamp FROM history ORDER BY id DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], |r| {
            Ok(HistoryRecord {
                id: Some(r.get(0)?),
                url: r.get(1)?,
                title: r.get(2)?,
                timestamp: Some(r.get(3)?),
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub fn clear_history(&self) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }

    pub fn insert_bookmark(&self, url: &str, title: &str) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO bookmarks (url, title) VALUES (?1, ?2)",
            params![url, title],
        )?;
        Ok(())
    }

    pub fn fetch_bookmarks(&self) -> rusqlite::Result<Vec<BookmarkRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, url, title FROM bookmarks ORDER BY id DESC")?;
        let rows = stmt.query_map([], |r| {
            Ok(BookmarkRecord {
                id: Some(r.get(0)?),
                url: r.get(1)?,
                title: r.get(2)?,
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub fn remove_bookmark(&self, id: i64) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM bookmarks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn save_config_item(&self, key: &str, val: &str) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, val],
        )?;
        Ok(())
    }

    pub fn load_config_item(&self, key: &str) -> Option<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT value FROM settings WHERE key = ?1")
            .ok()?;
        stmt.query_row(params![key], |r| r.get::<_, String>(0)).ok()
    }
}
