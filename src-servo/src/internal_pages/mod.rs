// src-servo/src/internal_pages/mod.rs
//! Quản lý trang nội bộ (`kestrel://`).

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
    let clean = url
        .trim()
        .split(&['?', '#'][..])
        .next()
        .unwrap_or("")
        .trim_end_matches('/')
        .to_ascii_lowercase();

    match clean.as_str() {
        "kestrel://home" | "kestrel://newtab" | "kestrel://" | "kestrel:" => Some(Page::Home),
        "kestrel://settings" | "kestrel://about" => Some(Page::Settings),
        "kestrel://history" => Some(Page::History),
        "kestrel://bookmarks" => Some(Page::Bookmarks),
        _ => None,
    }
}

pub fn is_internal(url: &str) -> bool {
    classify(url).is_some()
}

pub fn query_string(url: &str) -> Option<&str> {
    let without_hash = url.split('#').next().unwrap_or(url);
    without_hash.find('?').map(|i| &without_hash[i + 1..])
}

pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn sanitize_href(url: &str) -> String {
    let trimmed = url.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("https://")
        || lower.starts_with("http://")
        || lower.starts_with("kestrel://")
    {
        trimmed.to_string()
    } else if lower.starts_with("javascript:")
        || lower.starts_with("data:")
        || lower.starts_with("vbscript:")
        || lower.starts_with("file:")
    {
        "about:blank".to_string()
    } else {
        format!("https://{}", trimmed)
    }
}

/// JS shim: gửi thông điệp từ trang nội bộ về tiến trình chính Kestrel qua console IPC.
pub const NAV_SHIM: &str = r#"
(function () {
  var PREFIX = '__servo_gtk_msg__';

  window.__kestrelSend = function (payload) {
    try {
      if (window.servoGtk && window.servoGtk.messageHandlers &&
          window.servoGtk.messageHandlers.kestrel) {
        window.servoGtk.messageHandlers.kestrel.postMessage(payload);
        return true;
      }
      console.log(PREFIX + JSON.stringify({ name: 'kestrel', body: payload }));
      return true;
    } catch (_) {
      return false;
    }
  };

  function sendNav(url) {
    return window.__kestrelSend({ action: 'navigate', url: url });
  }

  document.addEventListener('click', function (e) {
    if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) {
      return;
    }
    var el = e.target;
    while (el && (!el.tagName || el.tagName.toUpperCase() !== 'A')) {
      el = el.parentElement;
    }
    if (!el) return;
    var href = (el.getAttribute('href') || '').trim();
    if (!href || href.charAt(0) === '#' || href.toLowerCase().indexOf('javascript:') === 0) {
      return;
    }
    if (href.indexOf('kestrel://') === 0 || href.indexOf('http://') === 0 || href.indexOf('https://') === 0) {
      e.preventDefault();
      sendNav(href);
    }
  }, true);
})();
"#;
