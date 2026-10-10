//! Quản lý trang nội bộ (kestrel://home, kestrel://settings, kestrel://history).

pub mod history;
pub mod home;
pub mod settings;

/// Các trang nội bộ được hỗ trợ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Settings,
    History,
}

/// Phân loại URL, trả về `Page` nếu là trang nội bộ.
pub fn classify(url: &str) -> Option<Page> {
    let t = url.trim().trim_end_matches('/');
    match t {
        "kestrel://home" | "kestrel://" | "kestrel:" => Some(Page::Home),
        "kestrel://settings" => Some(Page::Settings),
        "kestrel://history" => Some(Page::History),
        _ => None,
    }
}

/// Kiểm tra URL có phải trang nội bộ hay không.
pub fn is_internal(url: &str) -> bool {
    classify(url).is_some()
}
