//! Trang chủ nội bộ — kestrel://home.
//! Sử dụng Flexbox tương thích tuyệt đối với Servo và bắt trực tiếp phím Enter trên ô tìm kiếm.

use crate::internal_pages::NAV_SHIM;

pub fn render(
    lang: &str,
    dark: bool,
    bookmark_count: usize,
    history_count: usize,
    adblock_level: u8,
) -> String {
    let vi = lang == "vi";
    let theme_class = if dark { "dark" } else { "light" };

    let t = |en: &str, vn: &str| -> String {
        if vi { vn.to_string() } else { en.to_string() }
    };

    let shield_status = match adblock_level {
        0 => t("Off", "Tắt"),
        2 => t("Aggressive", "Mạnh"),
        _ => t("Standard", "Tiêu chuẩn"),
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="{lang}" class="{theme_class}">
<head>
<meta charset="utf-8">
<title>Kestrel — {title}</title>
<style>
  :root {{
    --bg: #101116;
    --fg: #e6e8f0;
    --muted: #828796;
    --card: #161820;
    --card-hover: #1c1f2a;
    --border: #242734;
    --border-hover: #3b4054;
  }}
  html.light {{
    --bg: #f5f6f8;
    --fg: #16181f;
    --muted: #606575;
    --card: #ffffff;
    --card-hover: #f8f9fb;
    --border: #dfe2ea;
    --border-hover: #b8becf;
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Inter, Roboto, sans-serif;
    background-color: var(--bg);
    color: var(--fg);
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 84px 24px 32px;
  }}
  .brand {{
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 6px;
  }}
  .logo {{
    font-size: 42px;
    font-weight: 700;
    letter-spacing: -1.2px;
    color: var(--fg);
  }}
  .tagline {{
    color: var(--muted);
    font-size: 13.5px;
    margin-bottom: 34px;
    text-align: center;
  }}
  .search-form {{
    width: 100%;
    max-width: 560px;
    display: flex;
    align-items: center;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 26px;
    padding: 4px 6px 4px 18px;
    margin-bottom: 34px;
  }}
  .search-form:focus-within {{
    border-color: var(--border-hover);
  }}
  .search-icon {{
    color: var(--muted);
    margin-right: 10px;
    flex-shrink: 0;
  }}
  .search-input {{
    flex: 1 1 auto;
    width: 100%;
    padding: 10px 4px;
    font-size: 14.5px;
    border: none;
    background: transparent;
    color: var(--fg);
    outline: none;
  }}
  .search-btn {{
    background: transparent;
    border: none;
    color: var(--muted);
    width: 34px;
    height: 34px;
    border-radius: 17px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }}
  .search-btn:hover {{
    background: var(--card-hover);
    color: var(--fg);
  }}
  .cards {{
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    width: 100%;
    max-width: 680px;
    margin: -6px;
  }}
  .card {{
    flex: 1 1 145px;
    max-width: 158px;
    margin: 6px;
    padding: 16px 14px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 12px;
    text-decoration: none;
    color: var(--fg);
    display: flex;
    flex-direction: column;
  }}
  .card:hover {{
    background: var(--card-hover);
    border-color: var(--border-hover);
  }}
  .card-top {{
    display: flex;
    align-items: center;
    margin-bottom: 8px;
    color: var(--muted);
  }}
  .card-title {{
    font-size: 13.5px;
    font-weight: 600;
    margin-bottom: 3px;
  }}
  .card-desc {{
    font-size: 12px;
    color: var(--muted);
    line-height: 1.35;
  }}
  .footer {{
    margin-top: auto;
    padding-top: 48px;
    font-size: 12px;
    color: var(--muted);
    display: flex;
    gap: 16px;
  }}
</style>
</head>
<body>
  <div class="brand">
    <div class="logo">Kestrel</div>
  </div>
  <div class="tagline">{tagline}</div>

  <form id="home-search-form" class="search-form" action="https://search.brave.com/search" method="get">
    <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>
    </svg>
    <input id="home-search-input" class="search-input" type="text" name="q" placeholder="{search_placeholder}" autocomplete="off" autofocus>
    <button id="home-search-btn" class="search-btn" type="submit" title="Search">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>
      </svg>
    </button>
  </form>

  <div class="cards">
    <a class="card" href="kestrel://bookmarks">
      <div class="card-top">
        <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"/></svg>
      </div>
      <div class="card-title">{bookmarks_card}</div>
      <div class="card-desc">{bookmark_count} {saved_lbl}</div>
    </a>
    <a class="card" href="kestrel://history">
      <div class="card-top">
        <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
      </div>
      <div class="card-title">{history_card}</div>
      <div class="card-desc">{history_count} {visited_lbl}</div>
    </a>
    <a class="card" href="kestrel://settings">
      <div class="card-top">
        <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
      </div>
      <div class="card-title">{settings_card}</div>
      <div class="card-desc">Shields: {shield_status}</div>
    </a>
    <a class="card" href="https://servo.org">
      <div class="card-top">
        <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
      </div>
      <div class="card-title">Servo.org</div>
      <div class="card-desc">{servo_desc}</div>
    </a>
  </div>

  <div class="footer">
    <span>Kestrel Browser</span>
    <span>·</span>
    <span>Pure Rust &amp; Servo</span>
  </div>

  <script>
    {shim}
    (function () {{
      var form = document.getElementById('home-search-form');
      var input = document.getElementById('home-search-input');
      var btn = document.getElementById('home-search-btn');

      function triggerSearch(e) {{
        if (!input) return;
        var q = input.value.trim();
        if (!q) {{
          if (e) e.preventDefault();
          return;
        }}
        if (window.servoGtk && window.servoGtk.messageHandlers &&
            window.servoGtk.messageHandlers.kestrel) {{
          if (e) e.preventDefault();
          window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify({{
            action: 'navigate',
            url: q
          }}));
        }}
      }}

      if (form) {{
        form.addEventListener('submit', triggerSearch);
      }}
      if (btn) {{
        btn.addEventListener('click', triggerSearch);
      }}
      if (input) {{
        input.addEventListener('keydown', function (e) {{
          if (e.key === 'Enter' || e.keyCode === 13) {{
            triggerSearch(e);
          }}
        }});
      }}
    }})();
  </script>
</body>
</html>"#,
        lang = lang,
        theme_class = theme_class,
        title = t("New Tab", "Tab mới"),
        tagline = t(
            "Quiet, pure-Rust web browser",
            "Trình duyệt thuần Rust tĩnh lặng và riêng tư"
        ),
        search_placeholder = t(
            "Search the web or enter URL...",
            "Tìm kiếm hoặc nhập địa chỉ URL..."
        ),
        bookmarks_card = t("Bookmarks", "Dấu trang"),
        bookmark_count = bookmark_count,
        saved_lbl = t("saved", "đã lưu"),
        history_card = t("History", "Lịch sử"),
        history_count = history_count,
        visited_lbl = t("visited", "đã xem"),
        settings_card = t("Settings", "Cài đặt"),
        shield_status = shield_status,
        servo_desc = t("Browser engine", "Nhân trình duyệt"),
        shim = NAV_SHIM,
    )
}
