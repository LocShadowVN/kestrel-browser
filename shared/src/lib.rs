use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HistoryRecord {
    pub id: Option<i64>,
    pub url: String,
    pub title: String,
    pub timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BookmarkRecord {
    pub id: Option<i64>,
    pub url: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AppConfig {
    pub search_engine: String,
    pub homepage: String,
    pub dark_theme: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            search_engine: "https://search.brave.com/search?q=".into(),
            homepage: "kestrel://home".into(),
            dark_theme: true,
        }
    }
}
