use rusqlite::{params, Connection};
use shared::{AppConfig, BookmarkRecord, HistoryRecord};
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
        fs::create_dir_all(&data_dir).expect("Cannot create app dir");

        let db_path = data_dir.join("kestrel.sqlite");
        let conn = Connection::open(&db_path).expect("SQLite init failed");

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
            ",
        )
        .expect("Schema migration failed");

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

    pub fn fetch_history(&self) -> rusqlite::Result<Vec<HistoryRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, url, title, timestamp FROM history ORDER BY id DESC LIMIT 100",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(HistoryRecord {
                id: Some(r.get(0)?),
                url: r.get(1)?,
                title: r.get(2)?,
                timestamp: Some(r.get(3)?),
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
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

    pub fn save_config_item(&self, key: &str, val: &str) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, val],
        )?;
        Ok(())
    }

    pub fn load_config(&self) -> AppConfig {
        let conn = self.conn.lock().unwrap();
        let mut cfg = AppConfig::default();

        if let Ok(mut stmt) = conn.prepare("SELECT key, value FROM settings") {
            if let Ok(rows) = stmt.query_map([], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
            }) {
                for (k, v) in rows.flatten() {
                    match k.as_str() {
                        "search_engine" => cfg.search_engine = v,
                        "homepage" => cfg.homepage = v,
                        "download_path" => cfg.download_path = v,
                        "dark_theme" => cfg.dark_theme = v == "true",
                        _ => {}
                    }
                }
            }
        }
        cfg
    }
}
