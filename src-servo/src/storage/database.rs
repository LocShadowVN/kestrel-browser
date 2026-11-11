// src-servo/src/storage/database.rs
//! Lưu trữ SQLite cục bộ với cơ chế tự động phục hồi (fallback sang In-Memory DB)
//! nếu hệ thống tệp bị khóa hoặc không có quyền ghi — tuân thủ Zero-Panic Policy.

use rusqlite::{params, Connection};
use shared::{BookmarkRecord, HistoryRecord};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

const SCHEMA_SQL: &str = "
    PRAGMA journal_mode = WAL;
    PRAGMA synchronous = NORMAL;
    PRAGMA temp_store = MEMORY;
    PRAGMA foreign_keys = ON;
    PRAGMA busy_timeout = 5000;

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
    CREATE TABLE IF NOT EXISTS session_tabs (
        position INTEGER PRIMARY KEY,
        url TEXT NOT NULL,
        title TEXT NOT NULL,
        is_active INTEGER NOT NULL DEFAULT 0
    );
    CREATE INDEX IF NOT EXISTS idx_history_url ON history(url);
";

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn init() -> Self {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("kestrel-browser");

        let conn = match Self::try_open_disk(&data_dir) {
            Ok((c, path)) => {
                log::info!("Database initialized at {:?}", path);
                c
            }
            Err(e) => {
                log::error!(
                    "Failed to open disk SQLite database in {:?}: {}. Falling back to in-memory database.",
                    data_dir,
                    e
                );
                Self::open_fallback_memory()
            }
        };

        Self {
            conn: Mutex::new(conn),
        }
    }

    fn try_open_disk(data_dir: &PathBuf) -> Result<(Connection, PathBuf), String> {
        fs::create_dir_all(data_dir).map_err(|e| format!("create_dir_all failed: {e}"))?;
        let db_path = data_dir.join("kestrel.sqlite");
        let conn = Connection::open(&db_path).map_err(|e| format!("sqlite open failed: {e}"))?;
        conn.execute_batch(SCHEMA_SQL)
            .map_err(|e| format!("schema init failed: {e}"))?;
        Ok((conn, db_path))
    }

    fn open_fallback_memory() -> Connection {
        match Connection::open_in_memory() {
            Ok(conn) => {
                if let Err(e) = conn.execute_batch(SCHEMA_SQL) {
                    log::error!("In-memory schema initialization error: {e}");
                }
                conn
            }
            Err(e) => {
                log::error!("Critical SQLite in-memory open error: {e}");
                std::process::abort();
            }
        }
    }

    fn lock_conn(&self) -> MutexGuard<'_, Connection> {
        self.conn.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn insert_history(&self, url: &str, title: &str) -> rusqlite::Result<()> {
        let trimmed = url.trim();
        if trimmed.is_empty()
            || trimmed == "about:blank"
            || trimmed.to_ascii_lowercase().starts_with("kestrel:")
        {
            return Ok(());
        }

        let conn = self.lock_conn();

        let latest: Option<(i64, String)> = conn
            .query_row(
                "SELECT id, url FROM history ORDER BY id DESC LIMIT 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .ok();

        if let Some((last_id, last_url)) = latest {
            if last_url == trimmed {
                if !title.trim().is_empty() {
                    conn.execute(
                        "UPDATE history SET title = ?1, timestamp = CURRENT_TIMESTAMP WHERE id = ?2",
                        params![title, last_id],
                    )?;
                } else {
                    conn.execute(
                        "UPDATE history SET timestamp = CURRENT_TIMESTAMP WHERE id = ?1",
                        params![last_id],
                    )?;
                }
                return Ok(());
            }
        }

        let safe_title = if title.trim().is_empty() {
            trimmed
        } else {
            title
        };
        conn.execute(
            "INSERT INTO history (url, title) VALUES (?1, ?2)",
            params![trimmed, safe_title],
        )?;
        Ok(())
    }

    pub fn fetch_history(&self, limit: usize) -> rusqlite::Result<Vec<HistoryRecord>> {
        let conn = self.lock_conn();
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
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn delete_history_item(&self, id: i64) -> rusqlite::Result<()> {
        let conn = self.lock_conn();
        conn.execute("DELETE FROM history WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn clear_history(&self) -> rusqlite::Result<()> {
        let conn = self.lock_conn();
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }

    pub fn history_count(&self) -> usize {
        let conn = self.lock_conn();
        conn.query_row("SELECT COUNT(*) FROM history", [], |r| r.get::<_, i64>(0))
            .unwrap_or(0) as usize
    }

    pub fn insert_bookmark(&self, url: &str, title: &str) -> rusqlite::Result<()> {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return Ok(());
        }
        let safe_title = if title.trim().is_empty() {
            trimmed
        } else {
            title
        };
        let conn = self.lock_conn();
        conn.execute(
            "INSERT OR REPLACE INTO bookmarks (url, title) VALUES (?1, ?2)",
            params![trimmed, safe_title],
        )?;
        Ok(())
    }

    pub fn is_bookmarked(&self, url: &str) -> bool {
        let conn = self.lock_conn();
        conn.query_row(
            "SELECT 1 FROM bookmarks WHERE url = ?1 LIMIT 1",
            params![url.trim()],
            |_| Ok(()),
        )
        .is_ok()
    }

    pub fn remove_bookmark_by_url(&self, url: &str) -> rusqlite::Result<()> {
        let conn = self.lock_conn();
        conn.execute("DELETE FROM bookmarks WHERE url = ?1", params![url.trim()])?;
        Ok(())
    }

    pub fn fetch_bookmarks(&self) -> rusqlite::Result<Vec<BookmarkRecord>> {
        let conn = self.lock_conn();
        let mut stmt = conn.prepare("SELECT id, url, title FROM bookmarks ORDER BY id DESC")?;
        let rows = stmt.query_map([], |r| {
            Ok(BookmarkRecord {
                id: Some(r.get(0)?),
                url: r.get(1)?,
                title: r.get(2)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn remove_bookmark(&self, id: i64) -> rusqlite::Result<()> {
        let conn = self.lock_conn();
        conn.execute("DELETE FROM bookmarks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn clear_bookmarks(&self) -> rusqlite::Result<()> {
        let conn = self.lock_conn();
        conn.execute("DELETE FROM bookmarks", [])?;
        Ok(())
    }

    pub fn bookmark_count(&self) -> usize {
        let conn = self.lock_conn();
        conn.query_row("SELECT COUNT(*) FROM bookmarks", [], |r| r.get::<_, i64>(0))
            .unwrap_or(0) as usize
    }

    pub fn save_config_item(&self, key: &str, val: &str) -> rusqlite::Result<()> {
        let conn = self.lock_conn();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, val],
        )?;
        Ok(())
    }

    pub fn load_config_item(&self, key: &str) -> Option<String> {
        let conn = self.lock_conn();
        let mut stmt = conn
            .prepare("SELECT value FROM settings WHERE key = ?1")
            .ok()?;
        stmt.query_row(params![key], |r| r.get::<_, String>(0)).ok()
    }

    /// Lưu danh sách các tab đang mở `(url, title, is_active)` để khôi phục phiên.
    pub fn save_session_tabs(&self, tabs: &[(String, String, bool)]) -> rusqlite::Result<()> {
        let mut conn = self.lock_conn();
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM session_tabs", [])?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO session_tabs (position, url, title, is_active) VALUES (?1, ?2, ?3, ?4)",
            )?;
            for (idx, (url, title, is_active)) in tabs.iter().enumerate() {
                stmt.execute(params![
                    idx as i64,
                    url.trim(),
                    title.trim(),
                    if *is_active { 1i64 } else { 0i64 }
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// Đọc danh sách các tab đã lưu từ phiên trước.
    pub fn load_session_tabs(&self) -> rusqlite::Result<Vec<(String, String, bool)>> {
        let conn = self.lock_conn();
        let mut stmt = conn.prepare(
            "SELECT url, title, is_active FROM session_tabs ORDER BY position ASC",
        )?;
        let rows = stmt.query_map([], |r| {
            let url: String = r.get(0)?;
            let title: String = r.get(1)?;
            let active: i64 = r.get(2)?;
            Ok((url, title, active != 0))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_db() -> Database {
        Database {
            conn: Mutex::new(Database::open_fallback_memory()),
        }
    }

    #[test]
    fn test_insert_and_fetch_history() {
        let db = temp_db();
        db.insert_history("https://example.com", "Example").unwrap();
        let rows = db.fetch_history(10).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].url, "https://example.com");
        assert_eq!(rows[0].title, "Example");
    }

    #[test]
    fn test_internal_pages_excluded_from_history() {
        let db = temp_db();
        db.insert_history("kestrel://home", "Home").unwrap();
        db.insert_history("kestrel://settings", "Settings").unwrap();
        assert_eq!(db.history_count(), 0);
    }

    #[test]
    fn test_consecutive_duplicate_history_deduped() {
        let db = temp_db();
        db.insert_history("https://example.com", "https://example.com")
            .unwrap();
        db.insert_history("https://example.com", "Example Domain")
            .unwrap();
        let rows = db.fetch_history(10).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].title, "Example Domain");
    }

    #[test]
    fn test_bookmark_roundtrip_and_url_delete() {
        let db = temp_db();
        db.insert_bookmark("https://rust-lang.org", "Rust").unwrap();
        assert!(db.is_bookmarked("https://rust-lang.org"));
        db.remove_bookmark_by_url("https://rust-lang.org").unwrap();
        assert!(!db.is_bookmarked("https://rust-lang.org"));
    }

    #[test]
    fn test_session_tabs_roundtrip() {
        let db = temp_db();
        let tabs = vec![
            ("https://servo.org".into(), "Servo".into(), false),
            ("https://rust-lang.org".into(), "Rust".into(), true),
        ];
        db.save_session_tabs(&tabs).unwrap();
        let loaded = db.load_session_tabs().unwrap();
        assert_eq!(loaded, tabs);
    }
}
