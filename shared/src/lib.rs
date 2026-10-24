use serde::{Deserialize, Serialize};

pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

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
    pub language: String,
    /// Adblock level: 0 = Off, 1 = Standard, 2 = Aggressive.
    pub adblock_level: u8,
    /// Custom User-Agent string (defaults to Desktop Chrome on Linux).
    pub user_agent: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            search_engine: "https://search.brave.com/search?q=".into(),
            homepage: "kestrel://home".into(),
            dark_theme: true,
            language: "en".into(),
            adblock_level: 1,
            user_agent: DEFAULT_USER_AGENT.into(),
        }
    }
}
