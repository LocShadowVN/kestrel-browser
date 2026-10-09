//! Quản lý các trang nội bộ của trình duyệt (kestrel://home, kestrel://settings, ...).
//!
//! Vì `servo-gtk` chưa expose custom protocol handler API, các trang này được
//! render bằng `WebView::load_html()` thay vì đăng ký scheme thật. Tiền tố
//! `kestrel://` trong omnibox được xử lý như một convention.

pub mod home;
pub mod settings;
pub mod history;

/// Trả về nội dung HTML cho một trang nội bộ, hoặc `None` nếu URL không phải
/// là trang nội bộ được hỗ trợ.
pub fn resolve(url: &str) -> Option<&'static str> {
    let trimmed = url.trim().trim_end_matches('/');
    match trimmed {
        "kestrel://home" | "kestrel://" | "kestrel:" => Some(home::HTML),
        "kestrel://settings" => Some(settings::HTML),
        "kestrel://history" => Some(history::HTML),
        _ => None,
    }
}
