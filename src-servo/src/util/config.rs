//! App config — wrapper quanh Database để đọc/ghi settings.
//!
//! Cache trong RAM để tránh query SQLite mỗi lần đọc.

use std::cell::RefCell;
use std::rc::Rc;

use shared::AppConfig;

use crate::storage::database::Database;

pub struct Config {
    db: Rc<Database>,
    cached: RefCell<AppConfig>,
}

impl Config {
    pub fn new(db: Rc<Database>) -> Self {
        let cached = db.load_config();
        Self {
            db,
            cached: RefCell::new(cached),
        }
    }

    pub fn get(&self) -> AppConfig {
        self.cached.borrow().clone()
    }

    pub fn search_engine(&self) -> String {
        self.cached.borrow().search_engine.clone()
    }

    pub fn homepage(&self) -> String {
        self.cached.borrow().homepage.clone()
    }

    pub fn download_path(&self) -> String {
        self.cached.borrow().download_path.clone()
    }

    pub fn dark_theme(&self) -> bool {
        self.cached.borrow().dark_theme
    }

    pub fn set_search_engine(&self, value: &str) -> Result<(), String> {
        self.db
            .save_config_item("search_engine", value)
            .map_err(|e| e.to_string())?;
        self.cached.borrow_mut().search_engine = value.to_string();
        Ok(())
    }

    pub fn set_homepage(&self, value: &str) -> Result<(), String> {
        self.db
            .save_config_item("homepage", value)
            .map_err(|e| e.to_string())?;
        self.cached.borrow_mut().homepage = value.to_string();
        Ok(())
    }

    pub fn set_download_path(&self, value: &str) -> Result<(), String> {
        self.db
            .save_config_item("download_path", value)
            .map_err(|e| e.to_string())?;
        self.cached.borrow_mut().download_path = value.to_string();
        Ok(())
    }

    pub fn set_dark_theme(&self, value: bool) -> Result<(), String> {
        let s = if value { "true" } else { "false" };
        self.db
            .save_config_item("dark_theme", s)
            .map_err(|e| e.to_string())?;
        self.cached.borrow_mut().dark_theme = value;
        Ok(())
    }
}
