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
    let without_hash = url.split('#').next().unwrap_or(url);
    without_hash.find('?').map(|i| &without_hash[i + 1..])
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
      // Fallback trực tiếp qua console hook nếu handler shim chưa kịp chạy
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
