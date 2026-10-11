//! Quản lý trang nội bộ (kestrel://home, kestrel://settings, kestrel://history).

pub mod history;
pub mod home;
pub mod settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Settings,
    History,
}

/// Phân loại URL, bỏ qua query string.
pub fn classify(url: &str) -> Option<Page> {
    let path = url.trim().split('?').next().unwrap_or("");
    let path = path.trim_end_matches('/');
    match path {
        "kestrel://home" | "kestrel://" | "kestrel:" => Some(Page::Home),
        "kestrel://settings" => Some(Page::Settings),
        "kestrel://history" => Some(Page::History),
        _ => None,
    }
}

pub fn is_internal(url: &str) -> bool {
    classify(url).is_some()
}

/// Trích query string (phần sau dấu `?`), nếu có.
pub fn query_string(url: &str) -> Option<&str> {
    url.find('?').map(|i| &url[i + 1..])
}
