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

/// JS shim: intercept click lên link trên trang nội bộ và gửi message `navigate` về native
/// để đồng bộ hoàn toàn Omnibox, trạng thái bảo mật và lịch sử của tab.
pub const NAV_SHIM: &str = r#"
(function () {
  window.__kestrelSend = function (payload) {
    if (window.servoGtk && window.servoGtk.messageHandlers &&
        window.servoGtk.messageHandlers.kestrel) {
      window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify(payload));
      return true;
    }
    return false;
  };

  function sendNav(url) {
    return window.__kestrelSend({ action: 'navigate', url: url });
  }

  document.addEventListener('click', function (e) {
    if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) {
      return;
    }
    var el = e.target;
    while (el && el.tagName !== 'A') el = el.parentElement;
    if (!el) return;
    var href = (el.getAttribute('href') || '').trim();
    if (!href || href.charAt(0) === '#' || href.toLowerCase().indexOf('javascript:') === 0) {
      return;
    }
    if (href.indexOf('kestrel://') === 0 || href.indexOf('http://') === 0 || href.indexOf('https://') === 0) {
      e.preventDefault();
      if (!sendNav(href)) {
        window.location.href = href;
      }
    }
  }, true);
})();
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_pages() {
        assert_eq!(classify("kestrel://home"), Some(Page::Home));
        assert_eq!(classify("KESTREL://HOME/"), Some(Page::Home));
        assert_eq!(classify("kestrel://settings#about"), Some(Page::Settings));
        assert_eq!(classify("kestrel://bookmarks?q=rust"), Some(Page::Bookmarks));
        assert_eq!(classify("kestrel://history"), Some(Page::History));
        assert_eq!(classify("kestrel://about"), None);
        assert_eq!(classify("https://example.com"), None);
    }

    #[test]
    fn test_query_string() {
        assert_eq!(
            query_string("kestrel://settings?lang=vi#top"),
            Some("lang=vi")
        );
        assert_eq!(query_string("kestrel://home"), None);
    }
}
