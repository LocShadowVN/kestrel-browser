//! Quản lý trang nội bộ.

pub mod bookmarks;
pub mod history;
pub mod home;
pub mod settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Settings,
    History,
    Bookmarks,
}

pub fn classify(url: &str) -> Option<Page> {
    let path = url.trim().split('?').next().unwrap_or("");
    let path = path.trim_end_matches('/');
    match path {
        "kestrel://home" | "kestrel://" | "kestrel:" => Some(Page::Home),
        "kestrel://settings" => Some(Page::Settings),
        "kestrel://history" => Some(Page::History),
        "kestrel://bookmarks" => Some(Page::Bookmarks),
        _ => None,
    }
}

pub fn is_internal(url: &str) -> bool {
    classify(url).is_some()
}

pub fn query_string(url: &str) -> Option<&str> {
    url.find('?').map(|i| &url[i + 1..])
}

/// JS shim: intercept click lên link `kestrel://` và gửi message về native
/// thay vì để Servo navigate (sẽ fail vì scheme không tồn tại).
pub const NAV_SHIM: &str = r#"
(function () {
  function sendNav(url) {
    if (window.servoGtk && window.servoGtk.messageHandlers &&
        window.servoGtk.messageHandlers.kestrel) {
      window.servoGtk.messageHandlers.kestrel.postMessage(
        JSON.stringify({ action: 'navigate', url: url })
      );
      return true;
    }
    return false;
  }
  document.addEventListener('click', function (e) {
    var el = e.target;
    while (el && el.tagName !== 'A') el = el.parentElement;
    if (!el) return;
    var href = el.getAttribute('href') || '';
    if (href.indexOf('kestrel://') === 0) {
      e.preventDefault();
      if (!sendNav(href)) {
        console.error('[Kestrel] message channel unavailable');
      }
    }
  }, true);
  console.log('[Kestrel] nav shim loaded; servoGtk:',
    typeof window.servoGtk);
})();
"#;
