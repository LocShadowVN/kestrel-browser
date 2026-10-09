//! Quản lý các trang nội bộ của trình duyệt (kestrel://home, kestrel://settings, ...).
//!
//! Vì `servo-gtk` chưa expose custom protocol handler API, các trang này được
//! render bằng `WebView::load_html()` thay vì đăng ký scheme thật. Tiền tố
//! `kestrel://` trong omnibox được xử lý như một convention.

pub mod history;
pub mod home;
pub mod settings;

/// Trả về nội dung HTML tĩnh cho một trang nội bộ, hoặc `None` nếu URL không
/// phải là trang nội bộ được hỗ trợ hoặc là trang cần dữ liệu động.
///
/// Trang `kestrel://history` KHÔNG nằm trong hàm này vì nó cần dữ liệu từ
/// database. Xem [`resolve_dynamic`].
pub fn resolve(url: &str) -> Option<&'static str> {
    let trimmed = url.trim().trim_end_matches('/');
    match trimmed {
        "kestrel://home" | "kestrel://" | "kestrel:" => Some(home::HTML),
        "kestrel://settings" => Some(settings::HTML),
        _ => None,
    }
}

/// Trả về `true` nếu URL là trang nội bộ cần render động (từ database).
pub fn is_dynamic(url: &str) -> bool {
    let trimmed = url.trim().trim_end_matches('/');
    matches!(trimmed, "kestrel://history")
}

/// Trả về `true` nếu URL là bất kỳ trang nội bộ nào (tĩnh hoặc động).
pub fn is_internal(url: &str) -> bool {
    resolve(url).is_some() || is_dynamic(url)
}
