//! Config — wrapper quanh Database, cache trong RAM.

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
        let mut cached = AppConfig::default();
        if let Some(v) = db.load_config_item("search_engine") {
            cached.search_engine = v;
        }
        if let Some(v) = db.load_config_item("homepage") {
            cached.homepage = v;
        }
        if let Some(v) = db.load_config_item("dark_theme") {
            cached.dark_theme = v == "true";
        }
        if let Some(v) = db.load_config_item("language") {
            cached.language = v;
        }
        Self {
            db,
            cached: RefCell::new(cached),
        }
    }

    pub fn homepage(&self) -> String {
        self.cached.borrow().homepage.clone()
    }

    pub fn search_engine(&self) -> String {
        self.cached.borrow().search_engine.clone()
    }

    pub fn dark_theme(&self) -> bool {
        self.cached.borrow().dark_theme
    }

    pub fn language(&self) -> String {
        self.cached.borrow().language.clone()
    }

    pub fn set_homepage(&self, value: &str) -> Result<(), String> {
        self.db
            .save_config_item("homepage", value)
            .map_err(|e| e.to_string())?;
        self.cached.borrow_mut().homepage = value.to_string();
        Ok(())
    }

    pub fn set_search_engine(&self, value: &str) -> Result<(), String> {
        self.db
            .save_config_item("search_engine", value)
            .map_err(|e| e.to_string())?;
        self.cached.borrow_mut().search_engine = value.to_string();
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

    pub fn set_language(&self, value: &str) -> Result<(), String> {
        let v = if value == "vi" { "vi" } else { "en" };
        self.db
            .save_config_item("language", v)
            .map_err(|e| e.to_string())?;
        self.cached.borrow_mut().language = v.to_string();
        Ok(())
    }
}
