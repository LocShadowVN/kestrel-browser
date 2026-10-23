//! Trang chủ nội bộ — kestrel://home.
//! Thiết kế Dashboard kiểu Brave với thanh thống kê Shields trực quan và lối tắt truy cập nhanh.

use crate::internal_pages::NAV_SHIM;

#[allow(dead_code)]
pub fn render(
    lang: &str,
    dark: bool,
    bookmark_count: usize,
    history_count: usize,
    adblock_level: u8,
) -> String {
    render_with_stats(lang, dark, bookmark_count, history_count, adblock_level, 0)
}

pub fn render_with_stats(
    lang: &str,
    dark: bool,
    bookmark_count: usize,
    history_count: usize,
    adblock_level: u8,
    total_blocked: u64,
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

    // Ước tính băng thông tiết kiệm (~28 KB mỗi quảng cáo/tracker)
    let kb_saved = total_blocked.saturating_mul(28);
    let data_saved_str = if kb_saved >= 1024 {
        format!("{:.1} MB", (kb_saved as f64) / 1024.0)
    } else {
        format!("{} KB", kb_saved)
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
    --muted: #868b9e;
    --card: #171a22;
    --card-hover: #1e222d;
    --border: #252936;
    --border-hover: #3f465c;
  }}
  html.light {{
    --bg: #f4f5f8;
    --fg: #171920;
    --muted: #5d6273;
    --card: #ffffff;
    --card-hover: #f8f9fc;
    --border: #dcdfe8;
    --border-hover: #b4bacb;
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
    padding: 52px 24px 28px;
  }}
  .stats-strip {{
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 12px;
    width: 100%;
    max-width: 660px;
    margin-bottom: 36px;
  }}
  .stat-box {{
    flex: 1 1 145px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 13px 16px;
    text-decoration: none;
    color: var(--fg);
  }}
  .stat-box:hover {{
    border-color: var(--border-hover);
    background: var(--card-hover);
  }}
  .stat-val {{
    font-size: 21px;
    font-weight: 700;
    letter-spacing: -0.5px;
    margin-bottom: 2px;
  }}
  .stat-lbl {{
    font-size: 11.5px;
    color: var(--muted);
  }}
  .brand {{
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    margin-bottom: 6px;
  }}
  .logo {{
    font-size: 36px;
    font-weight: 700;
    letter-spacing: -1px;
    color: var(--fg);
  }}
  .tagline {{
    color: var(--muted);
    font-size: 13.5px;
    margin-bottom: 26px;
    text-align: center;
  }}
  .search-form {{
    width: 100%;
    max-width: 600px;
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
  .quick-grid {{
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    width: 100%;
    max-width: 660px;
    margin-bottom: 24px;
  }}
  .site-tile {{
    flex: 1 1 150px;
    display: flex;
    align-items: center;
    gap: 11px;
    padding: 12px 14px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 11px;
    color: var(--fg);
    text-decoration: none;
  }}
  .site-tile:hover {{
    background: var(--card-hover);
    border-color: var(--border-hover);
  }}
  .site-badge {{
    width: 30px;
    height: 30px;
    border-radius: 8px;
    background: var(--bg);
    border: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 13px;
    font-weight: 700;
    color: var(--fg);
    flex-shrink: 0;
  }}
  .site-info {{
    overflow: hidden;
  }}
  .site-name {{
    font-size: 13px;
    font-weight: 600;
  }}
  .site-domain {{
    font-size: 11px;
    color: var(--muted);
  }}
  .footer {{
    margin-top: auto;
    padding-top: 32px;
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
  <div class="stats-strip">
    <a class="stat-box" href="kestrel://settings">
      <div class="stat-val">{total_blocked}</div>
      <div class="stat-lbl">{blocked_lbl} ({shield_status})</div>
    </a>
    <a class="stat-box" href="kestrel://settings">
      <div class="stat-val">{data_saved}</div>
      <div class="stat-lbl">{saved_data_lbl}</div>
    </a>
    <a class="stat-box" href="kestrel://bookmarks">
      <div class="stat-val">{bookmark_count}</div>
      <div class="stat-lbl">{bookmarks_card} (Ctrl+B)</div>
    </a>
    <a class="stat-box" href="kestrel://history">
      <div class="stat-val">{history_count}</div>
      <div class="stat-lbl">{history_card} (Ctrl+H)</div>
    </a>
  </div>

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

  <div class="section-label">{quick_lbl}</div>
  <div class="quick-grid">
    <a class="site-tile" href="https://github.com">
      <div class="site-badge">GH</div>
      <div class="site-info"><div class="site-name">GitHub</div><div class="site-domain">github.com</div></div>
    </a>
    <a class="site-tile" href="https://search.brave.com">
      <div class="site-badge">BR</div>
      <div class="site-info"><div class="site-name">Brave Search</div><div class="site-domain">search.brave.com</div></div>
    </a>
    <a class="site-tile" href="https://duckduckgo.com">
      <div class="site-badge">DD</div>
      <div class="site-info"><div class="site-name">DuckDuckGo</div><div class="site-domain">duckduckgo.com</div></div>
    </a>
    <a class="site-tile" href="https://vnexpress.net">
      <div class="site-badge">VN</div>
      <div class="site-info"><div class="site-name">VnExpress</div><div class="site-domain">vnexpress.net</div></div>
    </a>
    <a class="site-tile" href="https://en.wikipedia.org">
      <div class="site-badge">W</div>
      <div class="site-info"><div class="site-name">Wikipedia</div><div class="site-domain">wikipedia.org</div></div>
    </a>
    <a class="site-tile" href="https://news.ycombinator.com">
      <div class="site-badge">YC</div>
      <div class="site-info"><div class="site-name">Hacker News</div><div class="site-domain">news.ycombinator.com</div></div>
    </a>
    <a class="site-tile" href="https://www.rust-lang.org">
      <div class="site-badge">RS</div>
      <div class="site-info"><div class="site-name">Rust</div><div class="site-domain">rust-lang.org</div></div>
    </a>
    <a class="site-tile" href="kestrel://settings">
      <div class="site-badge">⚙</div>
      <div class="site-info"><div class="site-name">{settings_card}</div><div class="site-domain">kestrel://settings</div></div>
    </a>
  </div>

  <div class="footer">
    <span>Kestrel Browser</span>
    <span>·</span>
    <a href="kestrel://settings">{settings_card}</a>
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
        total_blocked = total_blocked,
        blocked_lbl = t("Ads & Trackers Blocked", "Quảng cáo & Tracker đã chặn"),
        data_saved = data_saved_str,
        saved_data_lbl = t("Est. Data Saved", "Dung lượng tiết kiệm"),
        tagline = t(
            "Fast, private web browser built on Rust & Servo",
            "Trình duyệt nhanh và riêng tư trên nền Rust & Servo"
        ),
        search_placeholder = t(
            "Search the web or type a URL...",
            "Tìm kiếm trên web hoặc nhập địa chỉ URL..."
        ),
        bookmarks_card = t("Bookmarks", "Dấu trang"),
        bookmark_count = bookmark_count,
        history_card = t("History", "Lịch sử"),
        history_count = history_count,
        settings_card = t("Settings", "Cài đặt"),
        quick_lbl = t("Quick Access", "Truy cập nhanh"),
        about_lbl = t("About", "Giới thiệu"),
        shield_status = shield_status,
        shim = NAV_SHIM,
    )
}
