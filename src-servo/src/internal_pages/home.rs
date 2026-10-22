//! Trang chủ nội bộ — kestrel://home.
//! Thiết kế tinh gọn, đầm mắt, hỗ trợ phím tắt `/` để focus ô tìm kiếm và lối tắt truy cập nhanh.

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
        0 => t("Shields Off", "Khiên: Tắt"),
        2 => t("Shields: Aggressive", "Khiên: Mạnh"),
        _ => t("Shields: Standard", "Khiên: Tiêu chuẩn"),
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="{lang}" class="{theme_class}">
<head>
<meta charset="utf-8">
<title>Kestrel — {title}</title>
<style>
  :root {{
    --bg: #111318;
    --fg: #e8eaf2;
    --muted: #84899c;
    --card: #171a22;
    --card-hover: #1d212c;
    --border: #252936;
    --border-hover: #3d4358;
    --pill: #1a1d27;
  }}
  html.light {{
    --bg: #f4f5f8;
    --fg: #171920;
    --muted: #5d6273;
    --card: #ffffff;
    --card-hover: #f8f9fc;
    --border: #dcdfe8;
    --border-hover: #b4bacb;
    --pill: #e9ecf2;
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
    padding: 72px 24px 28px;
  }}
  .top-pill {{
    display: inline-flex;
    align-items: center;
    gap: 7px;
    background: var(--pill);
    border: 1px solid var(--border);
    border-radius: 20px;
    padding: 5px 12px;
    font-size: 12px;
    color: var(--muted);
    margin-bottom: 18px;
    text-decoration: none;
  }}
  .top-pill:hover {{
    border-color: var(--border-hover);
    color: var(--fg);
  }}
  .brand {{
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    margin-bottom: 6px;
  }}
  .logo {{
    font-size: 38px;
    font-weight: 700;
    letter-spacing: -1px;
    color: var(--fg);
  }}
  .tagline {{
    color: var(--muted);
    font-size: 13.5px;
    margin-bottom: 30px;
    text-align: center;
  }}
  .search-form {{
    width: 100%;
    max-width: 580px;
    display: flex;
    align-items: center;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 14px;
    padding: 5px 8px 5px 16px;
    margin-bottom: 28px;
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
    padding: 9px 4px;
    font-size: 14.5px;
    border: none;
    background: transparent;
    color: var(--fg);
    outline: none;
  }}
  .kbd-hint {{
    font-family: monospace;
    font-size: 11px;
    color: var(--muted);
    border: 1px solid var(--border);
    border-radius: 5px;
    padding: 2px 6px;
    margin-right: 6px;
  }}
  .search-btn {{
    background: transparent;
    border: none;
    color: var(--muted);
    width: 34px;
    height: 34px;
    border-radius: 9px;
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
  .section-label {{
    width: 100%;
    max-width: 660px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.7px;
    color: var(--muted);
    margin: 4px 0 10px 4px;
  }}
  .cards {{
    display: flex;
    flex-wrap: wrap;
    width: 100%;
    max-width: 672px;
    margin: -6px -6px 22px -6px;
  }}
  .card {{
    flex: 1 1 150px;
    margin: 6px;
    padding: 15px 15px;
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
    justify-content: space-between;
    margin-bottom: 10px;
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
  }}
  .quick-links {{
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    width: 100%;
    max-width: 660px;
  }}
  .qlink {{
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 13px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 9px;
    color: var(--fg);
    text-decoration: none;
    font-size: 12.5px;
    font-weight: 500;
  }}
  .qlink:hover {{
    background: var(--card-hover);
    border-color: var(--border-hover);
  }}
  .qdot {{
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--muted);
  }}
  .footer {{
    margin-top: auto;
    padding-top: 40px;
    font-size: 12px;
    color: var(--muted);
    display: flex;
    gap: 14px;
    align-items: center;
  }}
  .footer a {{
    color: var(--muted);
    text-decoration: none;
  }}
  .footer a:hover {{
    color: var(--fg);
  }}
</style>
</head>
<body>
  <a class="top-pill" href="kestrel://settings">
    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
    <span>{shield_status}</span>
  </a>

  <div class="brand">
    <div class="logo">Kestrel</div>
  </div>
  <div class="tagline">{tagline}</div>

  <form id="home-search-form" class="search-form" action="https://search.brave.com/search" method="get">
    <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>
    </svg>
    <input id="home-search-input" class="search-input" type="text" name="q" placeholder="{search_placeholder}" autocomplete="off" autofocus>
    <span class="kbd-hint">/</span>
    <button id="home-search-btn" class="search-btn" type="submit" title="Go">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>
      </svg>
    </button>
  </form>

  <div class="section-label">{workspace_lbl}</div>
  <div class="cards">
    <a class="card" href="kestrel://bookmarks">
      <div class="card-top">
        <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"/></svg>
        <span class="kbd-hint">Ctrl+B</span>
      </div>
      <div class="card-title">{bookmarks_card}</div>
      <div class="card-desc">{bookmark_count} {saved_lbl}</div>
    </a>
    <a class="card" href="kestrel://history">
      <div class="card-top">
        <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
        <span class="kbd-hint">Ctrl+H</span>
      </div>
      <div class="card-title">{history_card}</div>
      <div class="card-desc">{history_count} {visited_lbl}</div>
    </a>
    <a class="card" href="kestrel://settings">
      <div class="card-top">
        <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
        <span class="kbd-hint">Ctrl+,</span>
      </div>
      <div class="card-title">{settings_card}</div>
      <div class="card-desc">{settings_desc}</div>
    </a>
  </div>

  <div class="section-label">{quick_lbl}</div>
  <div class="quick-links">
    <a class="qlink" href="https://github.com"><span class="qdot"></span>GitHub</a>
    <a class="qlink" href="https://duckduckgo.com"><span class="qdot"></span>DuckDuckGo</a>
    <a class="qlink" href="https://en.wikipedia.org"><span class="qdot"></span>Wikipedia</a>
    <a class="qlink" href="https://news.ycombinator.com"><span class="qdot"></span>Hacker News</a>
    <a class="qlink" href="https://vnexpress.net"><span class="qdot"></span>VnExpress</a>
    <a class="qlink" href="https://servo.org"><span class="qdot"></span>Servo.org</a>
  </div>

  <div class="footer">
    <span>Kestrel Browser</span>
    <span>·</span>
    <a href="kestrel://about">{about_lbl}</a>
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
        if (window.__kestrelSend && window.__kestrelSend({{ action: 'navigate', url: q }})) {{
          if (e) e.preventDefault();
        }}
      }}

      if (form) form.addEventListener('submit', triggerSearch);
      if (btn) btn.addEventListener('click', triggerSearch);
      if (input) {{
        input.addEventListener('keydown', function (e) {{
          if (e.key === 'Enter' || e.keyCode === 13) triggerSearch(e);
        }});
      }}

      document.addEventListener('keydown', function (e) {{
        if (e.key === '/' && document.activeElement !== input) {{
          e.preventDefault();
          if (input) {{ input.focus(); input.select(); }}
        }}
      }});
    }})();
  </script>
</body>
</html>"#,
        lang = lang,
        theme_class = theme_class,
        title = t("New Tab", "Tab mới"),
        tagline = t(
            "Lightweight, local-first web browser built on Rust & Servo",
            "Trình duyệt nhẹ, ưu tiên quyền riêng tư trên nền Rust & Servo"
        ),
        search_placeholder = t(
            "Search the web or type a URL...",
            "Tìm kiếm trên web hoặc nhập địa chỉ URL..."
        ),
        workspace_lbl = t("Browser", "Trình duyệt"),
        bookmarks_card = t("Bookmarks", "Dấu trang"),
        bookmark_count = bookmark_count,
        saved_lbl = t("saved pages", "trang đã lưu"),
        history_card = t("History", "Lịch sử"),
        history_count = history_count,
        visited_lbl = t("visited pages", "trang đã xem"),
        settings_card = t("Settings", "Cài đặt"),
        settings_desc = t("Theme, search & privacy", "Giao diện, tìm kiếm & bảo mật"),
        quick_lbl = t("Quick Access", "Truy cập nhanh"),
        about_lbl = t("About Kestrel", "Giới thiệu"),
        shield_status = shield_status,
        shim = NAV_SHIM,
    )
}
